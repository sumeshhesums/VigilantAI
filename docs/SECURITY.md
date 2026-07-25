# VigilantAI — Security Guide

> **Enterprise Security Intelligence Platform**
> Security Guide — v1.0

---

## Table of Contents

| Section | Title                                |
|---------|--------------------------------------|
| 1       | Authentication                       |
| 2       | Authorization (RBAC)                 |
| 3       | Password Security                    |
| 4       | CORS Configuration                   |
| 5       | Input Validation                     |
| 6       | SQL Injection Prevention             |
| 7       | File Upload Security                 |
| 8       | Secrets Management                   |
| 9       | Container Security                   |
| 10      | Network Security                     |
| 11      | Audit Logging                        |
| 12      | Vulnerability Management             |
| 13      | Compliance Considerations            |

---

## 1. Authentication

### 1.1 JWT RS256

VigilantAI uses asymmetric JWT signing with RS256 (RSA + SHA-256):

- **Private key** signs tokens (held by backend only)
- **Public key** verifies tokens (can be distributed)
- Enables key rotation without service restart

### 1.2 Token Lifecycle

| Token           | Lifetime    | Storage              | Rotation              |
|-----------------|-------------|----------------------|-----------------------|
| Access Token    | 15 minutes  | Client memory/header | New token on refresh  |
| Refresh Token   | 7 days      | httpOnly cookie      | Single-use, rotated   |

### 1.3 Token Claims

```json
{
  "sub": "user-uuid",
  "email": "user@vigilantai.com",
  "roles": ["security_analyst"],
  "sites": ["site-uuid-1"],
  "permissions": ["incidents.read"],
  "iss": "vigilantai",
  "aud": "vigilantai-api",
  "iat": 1719200000,
  "exp": 1719200900,
  "jti": "token-uuid"
}
```

### 1.4 Refresh Token Rotation

When a refresh token is used:

1. Server validates the refresh token hash against PostgreSQL
2. Server generates new access + refresh tokens
3. Old refresh token is marked as revoked
4. New refresh token is set in httpOnly cookie
5. If a revoked refresh token is presented, all tokens for that user are invalidated

### 1.5 Logout

**Current limitation:** Logout is a no-op — the access token remains valid until expiry.

**Planned mitigation:** Implement Redis-based JWT blocklist:

```
POST /api/v1/auth/logout
-> Add token JTI to Redis blocklist with TTL = token remaining expiry
-> Delete refresh token from PostgreSQL
```

---

## 2. Authorization (RBAC)

### 2.1 Role Hierarchy

| Role                 | Scope         | Key Capabilities                          |
|----------------------|---------------|-------------------------------------------|
| `system_admin`       | All sites     | Full administration, user/role management |
| `security_admin`     | All sites     | Camera/rule/incident management           |
| `security_analyst`   | Assigned sites| Investigation, evidence review            |
| `operator`           | Assigned sites| Alert acknowledgment, monitoring          |
| `viewer`             | Assigned sites| Read-only dashboard access                |
| `api_integration`    | Assigned sites| Programmatic API access                   |

### 2.2 Permission Enforcement

RBAC is enforced at the **Axum middleware layer**, before request handlers:

```
Request -> Rate Limiter -> JWT Validator -> RBAC Enforcer -> Audit Log -> Handler
```

The RBAC enforcer:

1. Extracts permissions from the JWT claims
2. Checks if the endpoint's required permission is in the user's permission set
3. Returns 403 Forbidden if unauthorized

### 2.3 Data Scope Filtering

Users are scoped to specific sites. All data queries are automatically filtered to return only data belonging to the user's authorized sites. System admins have global access across all sites.

### 2.4 Permission Matrix (23 Permissions)

| Action              | system_admin | security_admin | security_analyst | operator | viewer | api_integration |
|---------------------|:------------:|:--------------:|:----------------:|:--------:|:------:|:---------------:|
| Manage users        | Y            | Y (read)       | -                | -        | -      | -               |
| Manage roles        | Y            | -              | -                | -        | -      | -               |
| Manage cameras      | Y            | Y              | -                | -        | -      | -               |
| Create rules        | Y            | Y              | -                | -        | -      | -               |
| Toggle rules        | Y            | Y              | -                | -        | -      | -               |
| View cameras        | Y            | Y              | Y                | Y        | Y      | Y               |
| Create incidents    | Y            | Y              | Y                | Y        | -      | Y               |
| View incidents      | Y            | Y              | Y                | Y        | Y      | Y               |
| Upload evidence     | Y            | Y              | Y                | Y        | -      | Y               |
| Download evidence   | Y            | Y              | Y                | -        | -      | Y               |
| Acknowledge alerts  | Y            | Y              | Y                | Y        | -      | -               |
| View audit logs     | Y            | Y              | Y                | -        | -      | -               |
| Export audit logs   | Y            | Y              | -                | -        | -      | -               |
| System config       | Y            | -              | -                | -        | -      | -               |

---

## 3. Password Security

### 3.1 Hashing Algorithm

| Algorithm   | Parameters            | Usage             |
|-------------|-----------------------|-------------------|
| Argon2id    | t=3, m=65536, p=4    | Primary (preferred)|
| bcrypt      | cost=12               | Fallback          |

Argon2id is OWASP-recommended. It is memory-hard, resisting GPU and ASIC attacks.

### 3.2 Password Policy

| Policy             | Requirement                              |
|--------------------|------------------------------------------|
| Minimum length     | 12 characters                            |
| Maximum length     | 128 characters                           |
| Complexity         | Uppercase + lowercase + digit + special  |
| Password history   | Last 5 passwords rejected                |
| Breach check       | Have I Been Pwned k-anonymity API        |

### 3.3 Account Lockout

| Trigger                    | Threshold              | Duration     |
|----------------------------|------------------------|--------------|
| Failed login attempts      | 5 within 15 min        | 30 minutes   |
| Suspicious IP activity     | 10 within 1 hour       | 2 hours      |
| Multiple account lockouts  | 3 accounts from same IP| 24 hours     |

---

## 4. CORS Configuration

**Current limitation:** CORS is not configured in the backend. This is a known issue.

**Planned implementation:**

```rust
let cors = CorsLayer::new()
    .allow_origin([
        "http://localhost:3000".parse().unwrap(),
        "https://app.vigilantai.com".parse().unwrap(),
    ])
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::PATCH, Method::DELETE])
    .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE, header::ACCEPT])
    .max_age(Duration::from_secs(3600));
```

**Configuration via environment variable:** `CORS_ORIGINS`

---

## 5. Input Validation

### 5.1 Request Validation

All API requests are validated at the handler layer using Serde deserialization:

- **Required fields** — Missing fields return 400 Validation Error
- **Type checking** — Wrong types return 400 Validation Error
- **Enum validation** — Invalid enum values return 400 Validation Error
- **String length** — Enforced via Serde bounds
- **Email format** — Validated on user creation/update

### 5.2 SQL Injection Prevention

All database queries use **SQLx parameterized queries**. SQLx validates queries at compile time against the database schema:

```rust
// CORRECT: Parameterized query
sqlx::query("SELECT * FROM users WHERE id = $1")
    .bind(user_id)
    .fetch_one(&pool)
    .await?;
```

String interpolation in SQL is a compile-time error in SQLx.

### 5.3 Path Traversal Prevention

File paths for evidence storage are constructed server-side using UUIDs, not user input:

```rust
let path = format!("{}/{}/{}.{}", site_id, date, uuid, extension);
```

---

## 6. File Upload Security

### 6.1 Constraints

| Constraint              | Value                          | Enforcement          |
|-------------------------|--------------------------------|----------------------|
| Max file size           | 20 MB                          | Request middleware    |
| Allowed types           | JPEG, PNG, MP4                 | File type validation |
| Path                    | Server-generated (UUID-based)  | No user path input   |
| Storage location        | Configurable via `EVIDENCE_STORAGE_PATH` | Docker volume mount |

### 6.2 Integrity Verification

Every uploaded evidence file receives a SHA-256 content hash. The hash is stored in the database and verified on every download/access.

### 6.3 Directory Traversal Protection

- User input never directly constructs file paths
- File names are generated as `{uuid}.{extension}`
- Base directory is configurable and non-writable by users
- Symlinks are not followed

---

## 7. Secrets Management

### 7.1 Environment Variables

All secrets are stored as environment variables — never in source code:

| Secret                | Variable                  | Description                    |
|-----------------------|---------------------------|--------------------------------|
| Database password     | `POSTGRES_PASSWORD`       | PostgreSQL authentication      |
| Database URL          | `DATABASE_URL`            | Full connection string         |
| JWT signing key       | `JWT_PRIVATE_KEY`         | RSA private key for signing    |
| JWT verification key  | `JWT_PUBLIC_KEY`          | RSA public key for verification|
| Internal API key      | `GATEWAY_AUTH_TOKEN`      | Gateway-to-backend auth        |
| Redis password        | `REDIS_PASSWORD`          | Redis AUTH command             |

### 7.2 Key Generation

```bash
# Generate RSA 4096-bit key pair
openssl genrsa 4096 2>/dev/null | openssl pkcs8 -topk8 -nocrypt -outform PEM > jwt_private.pem
openssl rsa -in jwt_private.pem -pubout -out jwt_public.pem
```

### 7.3 Key Rotation

- Rotate JWT keys every 90 days (configurable)
- During rotation, deploy both old and new public keys
- After all tokens signed with old key expire, remove old key
- Never commit keys to version control

---

## 8. Container Security

### 8.1 Non-Root Execution

All containers run as non-root user (UID 1000). Dockerfiles include:

```dockerfile
RUN addgroup --system --gid 1000 appgroup && \
    adduser --system --uid 1000 --ingroup appgroup appuser
USER appuser
```

### 8.2 Read-Only Filesystem

Containers use read-only root filesystems where possible:

- Application code: read-only
- Temp directories: tmpfs mounts
- Evidence storage: dedicated writable volume only
- Logs: stdout/stderr (collected by Promtail)

### 8.3 Resource Limits

| Container         | CPU Limit | Memory Limit |
|-------------------|-----------|-------------|
| backend           | 4 cores   | 4 GB        |
| ai-service        | 8 cores   | 16 GB       |
| camera-gateway    | 4 cores   | 4 GB        |
| dashboard         | 0.5 cores | 256 MB      |
| postgres          | 4 cores   | 16 GB       |
| redis             | 2 cores   | 4 GB        |

### 8.4 Image Security

- Base images from official Docker Hub verified publishers
- Trivy vulnerability scanning on every build
- Multi-stage builds to minimize attack surface
- No shell access in production images where possible

---

## 9. Network Security

### 9.1 Network Segmentation

| Zone        | Subnet       | Purpose                        | Ingress                    |
|-------------|-------------|--------------------------------|----------------------------|
| Public      | Internet     | User access, camera ingestion  | HTTP/HTTPS (443), RTSP (554)|
| Application | 10.0.20.0/24| API processing, AI inference   | From DMZ only              |
| Data        | 10.0.30.0/24| Database, evidence, cache      | From Application only      |
| Management  | 10.0.40.0/24| Monitoring, logging, secrets   | Read-only from Application |

### 9.2 Internal Communication

| Path                     | Protocol  | Auth Method         |
|--------------------------|-----------|---------------------|
| Client <-> Load Balancer | HTTPS     | TLS 1.3             |
| Load Balancer <-> API    | HTTP      | Network isolation   |
| API <-> PostgreSQL       | PostgreSQL| SCRAM-SHA-256       |
| API <-> Redis            | Redis     | AUTH command        |
| Gateway <-> AI Service   | HTTP      | Internal API key    |
| Gateway <-> Backend      | HTTP      | Internal API key    |

### 9.3 TLS Configuration

- External traffic: TLS 1.3 (Let's Encrypt or custom CA)
- Internal services: mTLS (planned for production)
- Database connections: TLS 1.3 with SCRAM-SHA-256
- No plain HTTP in production

---

## 10. Audit Logging

### 10.1 Audit Trail Scope

Every state-changing operation generates an immutable audit log entry:

| Event Type        | Captured Data                                    |
|-------------------|--------------------------------------------------|
| Authentication    | Login success/failure, IP, user agent, timestamp |
| Authorization     | Permission check result, required permission     |
| CRUD operations   | User, action, resource, resource ID, changes     |
| Evidence access   | User, action, evidence ID, timestamp             |
| Configuration     | Setting changed, old value, new value, user      |
| System events     | Service start/stop, errors, migrations           |

### 10.2 Audit Log Immutability

- Audit logs use append-only storage at the database level
- `UPDATE` and `DELETE` operations are revoked on the audit_logs table
- Each entry includes: user ID, timestamp, IP address, action, resource, details

### 10.3 Structured Logging

All services emit JSON-structured logs via the `tracing` crate (Rust) or Python logging:

```json
{
  "timestamp": "2026-07-22T22:15:05.123Z",
  "level": "info",
  "service": "backend",
  "message": "Incident created",
  "incident_id": "inc-uuid-1",
  "user_id": "user-uuid-1",
  "correlation_id": "req-uuid-1",
  "duration_ms": 45
}
```

---

## 11. Vulnerability Management

### 11.1 Dependency Scanning

| Tool      | Language   | Frequency        | Scope                     |
|-----------|------------|------------------|---------------------------|
| cargo-audit| Rust      | Every build      | Cargo.lock dependencies   |
| pip-audit | Python     | Every build      | Python dependencies       |
| npm audit | TypeScript | Every build      | npm packages              |
| Trivy     | Containers | Every image build| Container image CVEs      |

### 11.2 Scan Procedure

```bash
# Rust dependency audit
cargo audit

# Python dependency audit
pip-audit

# npm audit
cd dashboard && npm audit

# Container image scan
trivy image vigilantai-backend:latest
trivy image vigilantai-ai-service:latest
```

### 11.3 Vulnerability Response SLA

| Severity    | Response Time | Remediation Time |
|-------------|---------------|------------------|
| Critical    | 1 hour        | 24 hours         |
| High        | 4 hours       | 72 hours         |
| Medium      | 24 hours      | 7 days           |
| Low         | 72 hours      | 30 days          |

---

## 12. Compliance Considerations

| Regulation  | Relevant Controls                                        |
|-------------|----------------------------------------------------------|
| GDPR        | Data retention policies, right to erasure (soft delete), audit trails, encryption at rest and in transit |
| CCPA        | Data access workflows, deletion requests, consent tracking |
| HIPAA       | Access controls, audit logging, encryption, business associate agreements |
| SOC 2       | Audit trails, change management, access controls, monitoring |
| ISO 27001   | ISMS alignment, risk management, continuous improvement  |

### 12.1 Data Retention

| Data Type        | Default Retention | Configurable | Deletion Method |
|------------------|-------------------|-------------|-----------------|
| Evidence files   | 90 days           | Yes         | Automatic purge |
| Audit logs       | 1 year            | Yes         | Archive then purge |
| Detection events | 30 days           | Yes         | Automatic purge |
| Incident data    | 2 years           | Yes         | Soft delete     |
| User accounts    | Until deactivated | N/A         | Soft delete     |

### 12.2 Encryption at Rest

| Data Type        | Encryption Method | Key Management     |
|------------------|-------------------|---------------------|
| Database         | PostgreSQL TDE    | Environment variable|
| Evidence files   | Filesystem-level  | OS/disk encryption  |
| Backups          | GPG encryption    | Offline key storage |
| Redis data       | Redis AUTH        | Environment variable|

### 12.3 Encryption in Transit

| Path                     | Protocol  | Version   |
|--------------------------|-----------|-----------|
| Client <-> API           | HTTPS     | TLS 1.3   |
| API <-> PostgreSQL       | PostgreSQL| TLS 1.3   |
| API <-> Redis            | Redis     | TLS (planned) |
| Internal services        | HTTP/mTLS | TLS 1.3 (planned) |
| Camera streams           | RTSP      | Optional  |
