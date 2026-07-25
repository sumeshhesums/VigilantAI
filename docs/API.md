# VigilantAI — API Reference

> **Enterprise Security Intelligence Platform**
> API Reference — v1.0

---

## Table of Contents

| Section | Title                                |
|---------|--------------------------------------|
| 1       | Overview                             |
| 2       | Authentication                       |
| 3       | Authorization (RBAC)                 |
| 4       | Request/Response Standards           |
| 5       | Error Handling                       |
| 6       | Pagination                           |
| 7       | Rate Limiting                        |
| 8       | Auth Endpoints                       |
| 9       | User Endpoints                       |
| 10      | Camera Endpoints                     |
| 11      | Incident Endpoints                   |
| 12      | Evidence Endpoints                   |
| 13      | Notification Endpoints               |
| 14      | Health Endpoints                     |
| 15      | Metrics Endpoint                     |
| 16      | WebSocket API                        |

---

## 1. Overview

### 1.1 Base URL

| Environment  | Base URL                           |
|--------------|------------------------------------|
| Development  | `http://localhost:8080/api/v1`     |
| Production   | `https://api.vigilantai.com/api/v1`|

### 1.2 Versioning

API version is embedded in the URL path: `/api/v1/...`. Breaking changes require a new version (`/api/v2/`). Additive changes (new fields, new endpoints) do not require version bumps.

### 1.3 Content Types

| Content Type              | Usage                                    |
|---------------------------|------------------------------------------|
| `application/json`        | All request and response bodies          |
| `multipart/form-data`     | Evidence file uploads                    |
| `application/octet-stream`| Evidence file downloads                  |

### 1.4 Common Headers

**Request Headers:**

| Header               | Required | Description                                |
|----------------------|----------|--------------------------------------------|
| `Authorization`      | Yes*     | `Bearer {access_token}` (except auth endpoints) |
| `Content-Type`       | Yes*     | `application/json` (for POST/PUT/PATCH)    |
| `X-Correlation-ID`   | No       | Client-provided correlation ID for tracing |
| `Idempotency-Key`    | No       | Unique key for POST idempotency            |

**Response Headers:**

| Header               | Description                                           |
|----------------------|-------------------------------------------------------|
| `X-Correlation-ID`   | Server-generated or echoed correlation ID             |
| `X-Request-ID`       | Unique request identifier                             |
| `X-RateLimit-Limit`  | Maximum requests per window                           |
| `X-RateLimit-Remaining` | Remaining requests in current window              |
| `X-RateLimit-Reset`  | Unix timestamp when rate limit window resets          |

---

## 2. Authentication

### 2.1 JWT RS256

All authenticated endpoints require a JWT access token in the `Authorization` header:

```
Authorization: Bearer eyJhbGciOiJSUzI1NiIs...
```

**Token lifecycle:**

```
Login → Access Token (15 min) + Refresh Token (7 days)
         │
         ├── Access Token expires → Use refresh token
         │                          │
         │                          ├── New access + refresh tokens issued
         │                          └── Old refresh token invalidated
         │
         └── Refresh Token expires → Re-authenticate
```

### 2.2 Token Refresh

When an access token expires, the client sends the refresh token to obtain new tokens:

```http
POST /api/v1/auth/refresh
Content-Type: application/json

{
  "refresh_token": "dGhpcyBpcyBhIHJlZnJl..."
}
```

Response:

```json
{
  "data": {
    "access_token": "eyJhbGciOiJSUzI1NiIs...",
    "refresh_token": "bmV3IHJlZnJlc2ggdG9r...",
    "token_type": "Bearer",
    "expires_in": 900
  }
}
```

### 2.3 JWT Claims

```json
{
  "sub": "user-uuid",
  "email": "operator@vigilantai.com",
  "roles": ["security_analyst"],
  "sites": ["site-uuid-1"],
  "permissions": ["incidents.read", "evidence.read"],
  "iss": "vigilantai",
  "aud": "vigilantai-api",
  "iat": 1719200000,
  "exp": 1719200900,
  "jti": "token-uuid"
}
```

---

## 3. Authorization (RBAC)

### 3.1 Roles

| Role                 | Description                               | Scope            |
|----------------------|-------------------------------------------|------------------|
| `system_admin`       | Full platform administration              | All sites        |
| `security_admin`     | Security operations management            | All sites        |
| `security_analyst`   | Alert monitoring, incident investigation  | Assigned sites   |
| `operator`           | Dashboard monitoring, rule management     | Assigned sites   |
| `viewer`             | Read-only access to dashboard and reports | Assigned sites   |
| `api_integration`    | API access for third-party integrations   | Assigned sites   |

### 3.2 Permissions

Permissions follow the pattern `{resource}:{action}`:

| Resource           | Actions                                                |
|--------------------|--------------------------------------------------------|
| `users`            | `create`, `read`, `update`, `delete`, `list`          |
| `roles`            | `create`, `read`, `update`, `delete`, `list`          |
| `cameras`          | `create`, `read`, `update`, `delete`, `list`          |
| `camera_groups`    | `create`, `read`, `update`, `delete`, `list`          |
| `rules`            | `create`, `read`, `update`, `delete`, `list`, `toggle`|
| `detection_events` | `read`, `list`                                         |
| `incidents`        | `create`, `read`, `update`, `list`, `assign`, `notes` |
| `evidence`         | `read`, `list`, `download`, `upload`                  |
| `alerts`           | `read`, `list`, `acknowledge`, `resolve`              |
| `notifications`    | `read`, `list`, `create`, `delete`                    |
| `audit_logs`       | `read`, `list`, `export`                              |
| `system_config`    | `read`, `update`                                       |

### 3.3 Permission Matrix

| Permission            | system_admin | security_admin | security_analyst | operator | viewer | api_integration |
|-----------------------|:------------:|:--------------:|:----------------:|:--------:|:------:|:---------------:|
| users:create          | ✅           | ✅             | ❌               | ❌       | ❌     | ❌              |
| users:read            | ✅           | ✅             | ✅ (own)         | ✅ (own) | ❌     | ❌              |
| users:delete          | ✅           | ❌             | ❌               | ❌       | ❌     | ❌              |
| cameras:read          | ✅           | ✅             | ✅               | ✅       | ✅     | ✅              |
| cameras:create        | ✅           | ✅             | ❌               | ❌       | ❌     | ❌              |
| incidents:create      | ✅           | ✅             | ✅               | ✅       | ❌     | ✅              |
| incidents:read        | ✅           | ✅             | ✅               | ✅       | ✅     | ✅              |
| evidence:read         | ✅           | ✅             | ✅               | ✅       | ❌     | ✅              |
| evidence:download     | ✅           | ✅             | ✅               | ❌       | ❌     | ✅              |
| alerts:acknowledge    | ✅           | ✅             | ✅               | ✅       | ❌     | ❌              |
| rules:create          | ✅           | ✅             | ❌               | ❌       | ❌     | ❌              |
| rules:toggle          | ✅           | ✅             | ❌               | ❌       | ❌     | ❌              |
| audit_logs:read       | ✅           | ✅             | ✅               | ❌       | ❌     | ❌              |
| system_config:update  | ✅           | ❌             | ❌               | ❌       | ❌     | ❌              |

---

## 4. Request/Response Standards

### 4.1 Success Response Envelope

**Single resource:**

```json
{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "name": "Main Lobby Camera",
    "status": "online",
    "created_at": "2026-07-22T10:00:00Z"
  }
}
```

**Paginated collection:**

```json
{
  "data": [
    { "id": "...", "name": "Camera 1" },
    { "id": "...", "name": "Camera 2" }
  ],
  "pagination": {
    "page": 1,
    "page_size": 20,
    "total_items": 150,
    "total_pages": 8
  }
}
```

### 4.2 Field Naming

- All fields use `snake_case`
- Timestamps in ISO 8601 UTC: `2026-07-22T10:00:00Z`
- IDs are UUID v4: `550e8400-e29b-41d4-a716-446655440000`

---

## 5. Error Handling

### 5.1 Error Response Format

```json
{
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "The request body contains invalid fields.",
    "details": [
      {
        "field": "email",
        "code": "INVALID_FORMAT",
        "message": "Must be a valid email address"
      }
    ],
    "request_id": "req-550e8400-e29b-41d4",
    "timestamp": "2026-07-22T10:00:00Z"
  }
}
```

### 5.2 Error Codes

| Code                       | HTTP Status | Description                         |
|----------------------------|:-----------:|-------------------------------------|
| `VALIDATION_ERROR`         | 400         | Invalid request body or parameters  |
| `AUTHENTICATION_REQUIRED`  | 401         | No authentication token provided    |
| `INVALID_TOKEN`            | 401         | Token expired, malformed, or invalid|
| `INVALID_CREDENTIALS`      | 401         | Wrong email or password             |
| `INSUFFICIENT_PERMISSIONS` | 403         | Authenticated but not authorized    |
| `RESOURCE_NOT_FOUND`       | 404         | Resource does not exist             |
| `RESOURCE_CONFLICT`        | 409         | Resource state conflict             |
| `RATE_LIMIT_EXCEEDED`      | 429         | Too many requests                   |
| `ACCOUNT_LOCKED`           | 423         | Account locked due to failed attempts|
| `INTERNAL_ERROR`           | 500         | Unexpected server error             |
| `SERVICE_UNAVAILABLE`      | 503         | Temporary outage                    |

---

## 6. Pagination

All list endpoints support cursor-based pagination:

```
GET /api/v1/cameras?page=1&page_size=20&sort_by=created_at&sort_order=desc
```

| Parameter    | Type   | Default | Description                     |
|--------------|--------|---------|---------------------------------|
| `page`       | int    | 1       | Page number                     |
| `page_size`  | int    | 20      | Items per page (max: 100)       |
| `sort_by`    | string | `created_at` | Sort field                 |
| `sort_order` | string | `desc`  | Sort direction (`asc` or `desc`)|

**Response pagination object:**

```json
{
  "pagination": {
    "page": 1,
    "page_size": 20,
    "total_items": 150,
    "total_pages": 8
  }
}
```

---

## 7. Rate Limiting

### 7.1 Default Limits

| Tier               | Requests/Minute | Requests/Hour | Burst     |
|--------------------|:---------------:|:-------------:|-----------|
| Standard User      | 100             | 3,000         | 20/sec    |
| API Integration    | 300             | 10,000        | 50/sec    |
| System Admin       | 200             | 6,000         | 30/sec    |

### 7.2 Rate Limit Response

```http
HTTP/1.1 429 Too Many Requests
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 0
X-RateLimit-Reset: 1690000060
Retry-After: 30

{
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Rate limit exceeded. Try again in 30 seconds.",
    "retry_after": 30
  }
}
```

---

## 8. Auth Endpoints

### 8.1 POST /api/v1/auth/login

Authenticate a user and receive JWT tokens.

**Request:**

```json
{
  "email": "operator@vigilantai.com",
  "password": "secure_password"
}
```

**Response (200):**

```json
{
  "data": {
    "access_token": "eyJhbGciOiJSUzI1NiIs...",
    "refresh_token": "dGhpcyBpcyBhIHJlZnJl...",
    "token_type": "Bearer",
    "expires_in": 900,
    "user": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "email": "operator@vigilantai.com",
      "first_name": "John",
      "last_name": "Operator",
      "roles": ["operator"],
      "site_ids": ["site-uuid-1"]
    }
  }
}
```

**Errors:**

| Status | Code                  | Condition                      |
|--------|-----------------------|--------------------------------|
| 401    | `INVALID_CREDENTIALS` | Wrong email or password        |
| 423    | `ACCOUNT_LOCKED`      | Too many failed login attempts |
| 422    | `VALIDATION_ERROR`    | Missing or malformed fields    |

### 8.2 POST /api/v1/auth/refresh

Refresh an access token using a refresh token.

**Request:**

```json
{
  "refresh_token": "dGhpcyBpcyBhIHJlZnJl..."
}
```

**Response (200):**

```json
{
  "data": {
    "access_token": "eyJhbGciOiJSUzI1NiIs...",
    "refresh_token": "bmV3IHJlZnJlc2ggdG9r...",
    "token_type": "Bearer",
    "expires_in": 900
  }
}
```

### 8.3 POST /api/v1/auth/logout

Invalidate the current session.

```http
POST /api/v1/auth/logout
Authorization: Bearer {access_token}
```

**Response:** `204 No Content`

### 8.4 GET /api/v1/auth/me

Get the current authenticated user's profile.

```http
GET /api/v1/auth/me
Authorization: Bearer {access_token}
```

**Response (200):**

```json
{
  "data": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "operator@vigilantai.com",
    "first_name": "John",
    "last_name": "Operator",
    "roles": ["operator"],
    "site_ids": ["site-uuid-1"],
    "last_login_at": "2026-07-22T09:30:00Z",
    "created_at": "2026-01-15T00:00:00Z"
  }
}
```

---

## 9. User Endpoints

### 9.1 GET /api/v1/users

List users with pagination and filtering.

```http
GET /api/v1/users?page=1&page_size=20&role=operator&status=active
Authorization: Bearer {access_token}
Required Permission: users:list
```

**Query Parameters:**

| Parameter    | Type   | Description                    |
|--------------|--------|--------------------------------|
| `page`       | int    | Page number                    |
| `page_size`  | int    | Items per page                 |
| `role`       | string | Filter by role name            |
| `status`     | string | Filter by status               |
| `search`     | string | Search by name or email        |
| `sort_by`    | string | Sort field                     |
| `sort_order` | string | `asc` or `desc`               |

### 9.2 GET /api/v1/users/{user_id}

Get a specific user.

```http
GET /api/v1/users/{user_id}
Authorization: Bearer {access_token}
Required Permission: users:read
```

### 9.3 POST /api/v1/users

Create a new user.

```http
POST /api/v1/users
Authorization: Bearer {access_token}
Required Permission: users:create
```

**Request:**

```json
{
  "email": "newuser@vigilantai.com",
  "first_name": "New",
  "last_name": "User",
  "password": "initial_password",
  "role_ids": ["role-uuid-operator"],
  "site_ids": ["site-uuid-1"]
}
```

**Response:** `201 Created`

### 9.4 PATCH /api/v1/users/{user_id}

Update a user.

```http
PATCH /api/v1/users/{user_id}
Authorization: Bearer {access_token}
Required Permission: users:update
```

### 9.5 DELETE /api/v1/users/{user_id}

Deactivate a user (soft delete).

```http
DELETE /api/v1/users/{user_id}
Authorization: Bearer {access_token}
Required Permission: users:delete
```

**Response:** `204 No Content`

---

## 10. Camera Endpoints

### 10.1 GET /api/v1/cameras

List cameras with filtering and pagination.

```http
GET /api/v1/cameras?site_id={site_id}&status=online&page=1&page_size=20
Authorization: Bearer {access_token}
Required Permission: cameras:list
```

### 10.2 GET /api/v1/cameras/{camera_id}

Get a specific camera.

```http
GET /api/v1/cameras/{camera_id}
Authorization: Bearer {access_token}
Required Permission: cameras:read
```

**Response (200):**

```json
{
  "data": {
    "id": "cam-uuid-1",
    "site_id": "site-uuid-1",
    "camera_group_id": "group-uuid-1",
    "name": "Main Lobby Camera",
    "rtsp_url": "rtsp://192.168.10.101:554/stream1",
    "status": "online",
    "fps": 15,
    "resolution_width": 1920,
    "resolution_height": 1080,
    "night_vision_enabled": true,
    "motion_detection_enabled": true,
    "storage_mode": "continuous",
    "created_at": "2026-01-15T00:00:00Z"
  }
}
```

### 10.3 POST /api/v1/cameras

Register a new camera.

```http
POST /api/v1/cameras
Authorization: Bearer {access_token}
Required Permission: cameras:create
```

**Request:**

```json
{
  "site_id": "site-uuid-1",
  "camera_group_id": "group-uuid-1",
  "name": "Parking Lot Camera",
  "rtsp_url": "rtsp://192.168.10.102:554/stream1",
  "fps": 15,
  "resolution_width": 1920,
  "resolution_height": 1080,
  "night_vision_enabled": true,
  "motion_detection_enabled": true,
  "storage_mode": "motion"
}
```

### 10.4 PATCH /api/v1/cameras/{camera_id}

Update camera configuration.

```http
PATCH /api/v1/cameras/{camera_id}
Authorization: Bearer {access_token}
Required Permission: cameras:update
```

### 10.5 DELETE /api/v1/cameras/{camera_id}

Remove a camera.

```http
DELETE /api/v1/cameras/{camera_id}
Authorization: Bearer {access_token}
Required Permission: cameras:delete
```

**Response:** `204 No Content`

---

## 11. Incident Endpoints

### 11.1 GET /api/v1/incidents

List incidents with filtering.

```http
GET /api/v1/incidents?status=open&severity=high&page=1&page_size=20
Authorization: Bearer {access_token}
Required Permission: incidents:list
```

**Query Parameters:**

| Parameter      | Type     | Description                              |
|----------------|----------|------------------------------------------|
| `status`       | string   | `open`, `acknowledged`, `investigating`, `resolved`, `closed` |
| `severity`     | string   | `critical`, `high`, `medium`, `low`     |
| `priority`     | string   | `p1`, `p2`, `p3`, `p4`                 |
| `assigned_to`  | uuid     | Filter by assigned user                  |
| `site_id`      | uuid     | Filter by site                           |
| `from`         | datetime | Start of time range (ISO 8601)           |
| `to`           | datetime | End of time range (ISO 8601)             |

### 11.2 GET /api/v1/incidents/{incident_id}

Get a specific incident with evidence and notes.

```http
GET /api/v1/incidents/{incident_id}
Authorization: Bearer {access_token}
Required Permission: incidents:read
```

### 11.3 POST /api/v1/incidents

Create a new incident.

```http
POST /api/v1/incidents
Authorization: Bearer {access_token}
Required Permission: incidents:create
```

**Request:**

```json
{
  "title": "Suspicious Activity - Parking Garage",
  "description": "Multiple persons detected in restricted area after hours",
  "severity": "high",
  "priority": "p2",
  "site_id": "site-uuid-1"
}
```

### 11.4 PATCH /api/v1/incidents/{incident_id}

Update incident status or details.

```http
PATCH /api/v1/incidents/{incident_id}
Authorization: Bearer {access_token}
Required Permission: incidents:update
```

### 11.5 POST /api/v1/incidents/{incident_id}/acknowledge

Acknowledge an incident.

```http
POST /api/v1/incidents/{incident_id}/acknowledge
Authorization: Bearer {access_token}
Required Permission: incidents:update
```

### 11.6 POST /api/v1/incidents/{incident_id}/resolve

Resolve an incident.

```http
POST /api/v1/incidents/{incident_id}/resolve
Authorization: Bearer {access_token}
Required Permission: incidents:update
```

---

## 12. Evidence Endpoints

### 12.1 GET /api/v1/evidence

List evidence records.

```http
GET /api/v1/evidence?incident_id={incident_id}&page=1&page_size=20
Authorization: Bearer {access_token}
Required Permission: evidence:list
```

### 12.2 GET /api/v1/evidence/{evidence_id}

Get evidence metadata.

```http
GET /api/v1/evidence/{evidence_id}
Authorization: Bearer {access_token}
Required Permission: evidence:read
```

### 12.3 POST /api/v1/evidence

Upload evidence (multipart/form-data).

```http
POST /api/v1/evidence
Authorization: Bearer {access_token}
Required Permission: evidence:upload
Content-Type: multipart/form-data
```

**Form Fields:**

| Field          | Type   | Required | Description               |
|----------------|--------|----------|---------------------------|
| `file`         | file   | Yes      | Evidence file (JPEG/PNG/MP4) |
| `incident_id`  | uuid   | Yes      | Associated incident       |
| `description`  | string | No       | Evidence description      |

**Constraints:**
- Max file size: 20 MB (`EVIDENCE_MAX_FILE_SIZE`)
- Allowed types: JPEG, PNG, MP4
- SHA-256 integrity hash computed on upload

### 12.4 GET /api/v1/evidence/{evidence_id}/download

Download evidence file.

```http
GET /api/v1/evidence/{evidence_id}/download
Authorization: Bearer {access_token}
Required Permission: evidence:download
```

**Response:** Binary file download with `Content-Type: application/octet-stream`

### 12.5 DELETE /api/v1/evidence/{evidence_id}

Delete evidence.

```http
DELETE /api/v1/evidence/{evidence_id}
Authorization: Bearer {access_token}
Required Permission: evidence:delete
```

---

## 13. Notification Endpoints

### 13.1 GET /api/v1/notifications

List notifications.

```http
GET /api/v1/notifications?status=pending&page=1&page_size=20
Authorization: Bearer {access_token}
Required Permission: notifications:list
```

### 13.2 GET /api/v1/notifications/{notification_id}

Get a specific notification.

```http
GET /api/v1/notifications/{notification_id}
Authorization: Bearer {access_token}
Required Permission: notifications:read
```

### 13.3 POST /api/v1/notifications

Send a notification.

```http
POST /api/v1/notifications
Authorization: Bearer {access_token}
Required Permission: notifications:create
```

**Request:**

```json
{
  "recipient_id": "user-uuid-1",
  "type": "alert",
  "title": "Critical Incident Created",
  "message": "Unauthorized access detected at Server Room",
  "channels": ["dashboard", "email"]
}
```

### 13.4 POST /api/v1/notifications/{notification_id}/retry

Retry a failed notification.

```http
POST /api/v1/notifications/{notification_id}/retry
Authorization: Bearer {access_token}
Required Permission: notifications:create
```

### 13.5 DELETE /api/v1/notifications/{notification_id}

Delete a notification.

```http
DELETE /api/v1/notifications/{notification_id}
Authorization: Bearer {access_token}
Required Permission: notifications:delete
```

---

## 14. Health Endpoints

### 14.1 GET /api/v1/health

Basic health check. No authentication required.

```http
GET /api/v1/health
```

**Response (200):**

```json
{
  "status": "healthy",
  "version": "1.0.0",
  "timestamp": "2026-07-22T10:00:00Z"
}
```

### 14.2 GET /api/v1/admin/health

Detailed health check including database and Redis connectivity. Requires authentication.

```http
GET /api/v1/admin/health
Authorization: Bearer {access_token}
```

**Response (200):**

```json
{
  "status": "healthy",
  "version": "1.0.0",
  "timestamp": "2026-07-22T10:00:00Z",
  "components": {
    "database": {
      "status": "healthy",
      "latency_ms": 2
    },
    "redis": {
      "status": "healthy",
      "latency_ms": 1
    },
    "evidence_storage": {
      "status": "healthy",
      "available_bytes": 107374182400
    }
  }
}
```

### 14.3 AI Service Health

```http
GET /health
```

**Response (200):**

```json
{
  "status": "healthy",
  "model_loaded": true,
  "model_name": "yolov8n",
  "device": "cpu"
}
```

### 14.4 Camera Gateway Health

```http
GET /health
```

**Response (200):**

```json
{
  "status": "healthy",
  "cameras_connected": 12,
  "cameras_online": 11,
  "cameras_offline": 1
}
```

---

## 15. Metrics Endpoint

### 15.1 GET /metrics

Prometheus-format metrics. No authentication required (internal network only).

```http
GET /metrics
```

**Response:** Prometheus text format

```
# HELP vigilantai_http_requests_total Total HTTP requests
# TYPE vigilantai_http_requests_total counter
vigilantai_http_requests_total{method="GET",endpoint="/api/v1/cameras",status="200"} 1234

# HELP vigilantai_http_request_duration_seconds HTTP request duration
# TYPE vigilantai_http_request_duration_seconds histogram
vigilantai_http_request_duration_seconds_bucket{method="GET",endpoint="/api/v1/cameras",le="0.1"} 1100
vigilantai_http_request_duration_seconds_bucket{method="GET",endpoint="/api/v1/cameras",le="0.5"} 1200
vigilantai_http_request_duration_seconds_bucket{method="GET",endpoint="/api/v1/cameras",le="1.0"} 1230
vigilantai_http_request_duration_seconds_bucket{method="GET",endpoint="/api/v1/cameras",le="+Inf"} 1234
```

---

## 16. WebSocket API

### 16.1 Connection

```javascript
const ws = new WebSocket('ws://localhost:8080/ws/v1?token={access_token}');
```

### 16.2 Events

| Event                | Direction   | Description                          |
|----------------------|-------------|--------------------------------------|
| `alert.new`          | Server→Client | New alert generated                |
| `alert.update`       | Server→Client | Alert status changed               |
| `incident.new`       | Server→Client | New incident created               |
| `incident.update`    | Server→Client | Incident status changed            |
| `camera.status`      | Server→Client | Camera online/offline transition   |
| `detection.event`    | Server→Client | New detection event                |
| `subscribe`          | Client→Server | Subscribe to event channels        |
| `unsubscribe`        | Client→Server | Unsubscribe from event channels    |
| `ping`               | Bidirectional | Keepalive heartbeat                |

### 16.3 Subscribe Message

```json
{
  "type": "subscribe",
  "channels": ["alerts", "incidents", "cameras"],
  "site_ids": ["site-uuid-1"]
}
```

### 16.4 Alert Event

```json
{
  "type": "alert.new",
  "data": {
    "id": "alert-uuid-1",
    "incident_id": "inc-uuid-1",
    "severity": "high",
    "title": "Unauthorized Access Detected",
    "camera_name": "Server Room Camera",
    "site_name": "Corporate HQ",
    "created_at": "2026-07-22T22:15:05Z"
  }
}
```
