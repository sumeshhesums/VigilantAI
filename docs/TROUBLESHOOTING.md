# VigilantAI — Troubleshooting Guide

> **Enterprise Security Intelligence Platform**
> Troubleshooting Guide — v1.0

---

## Table of Contents

| Section | Title                                |
|---------|--------------------------------------|
| 1       | General Troubleshooting Approach     |
| 2       | Docker Compose Issues                |
| 3       | Kubernetes Issues                    |
| 4       | Backend Service Issues               |
| 5       | AI Service Issues                    |
| 6       | Camera Gateway Issues                |
| 7       | Dashboard Issues                     |
| 8       | Database Issues                      |
| 9       | Redis Issues                         |
| 10      | JWT / Authentication Issues          |
| 11      | Monitoring Stack Issues              |
| 12      | Log Analysis Techniques              |
| 13      | Performance Tuning                   |

---

## 1. General Troubleshooting Approach

1. **Check service health:**
   ```bash
   make docker-ps
   curl http://localhost:8080/api/v1/health
   curl http://localhost:8081/health
   curl http://localhost:8082/health
   ```

2. **Check logs:**
   ```bash
   make docker-logs
   # Or for specific service:
   docker compose logs -f backend
   ```

3. **Check Prometheus alerts:** Open Grafana → Alerting panel

4. **Check resource usage:**
   ```bash
   docker stats
   ```

---

## 2. Docker Compose Issues

### 2.1 Services fail to start

**Symptom:** `docker compose up` shows services in `Exit` or `Restarting` state

**Diagnosis:**

```bash
# Check service status
docker compose ps

# Check specific service logs
docker compose logs backend
docker compose logs postgres
```

**Common causes:**

| Cause                              | Solution                                             |
|------------------------------------|------------------------------------------------------|
| PostgreSQL not ready               | `backend` depends on `postgres` healthcheck; wait for healthy state |
| Missing environment variables      | Ensure `.env` file exists with all required values  |
| JWT keys not configured            | Generate and set `JWT_PRIVATE_KEY` and `JWT_PUBLIC_KEY` in `.env` |
| Port conflict                      | Check if ports 8080/8081/8082/5432/6379 are already in use |
| Insufficient disk space           | `docker system df` → `docker system prune`          |

### 2.2 Port conflicts

**Symptom:** `Bind for 0.0.0.0:8080 failed: port is already allocated`

**Solution:**

```bash
# Find process using the port
netstat -tlnp | grep 8080
# Or on Windows
netstat -ano | findstr :8080

# Change port in .env
BACKEND_PORT=8090
```

### 2.3 Container exits immediately after start

**Diagnosis:**

```bash
docker compose logs --tail 50 <service-name>
```

**Common causes:**

| Cause                          | Solution                                      |
|--------------------------------|-----------------------------------------------|
| Database connection refused    | Ensure postgres container is healthy first    |
| Invalid JWT key format         | Regenerate keys using the correct commands    |
| Migration failure              | Check migration files and database state      |
| Missing model weights (AI)     | Set `AI_SERVICE_AUTO_LOAD=false` or mount weights |

### 2.4 Cannot build Docker images

**Symptom:** `docker compose build` fails

**Solution:**

```bash
# Ensure Docker has enough resources (Memory: 8GB+)
# Check Docker build context
docker compose build --no-cache

# For backend: ensure Rust toolchain is available in build stage
# Check Dockerfile syntax
```

---

## 3. Kubernetes Issues

### 3.1 Pods stuck in Pending

```bash
kubectl get pods -n vigilantai
kubectl describe pod <pod-name> -n vigilantai
```

**Common causes:**

| Cause                          | Solution                                      |
|--------------------------------|-----------------------------------------------|
| Insufficient resources         | Check node capacity: `kubectl describe nodes` |
| PVC not bound                  | Check PVC status: `kubectl get pvc -n vigilantai` |
| Node selector mismatch        | Verify node labels match pod requirements     |

### 3.2 Pods in CrashLoopBackOff

```bash
kubectl logs <pod-name> -n vigilantai --previous
```

**Common causes:**

| Cause                          | Solution                                      |
|--------------------------------|-----------------------------------------------|
| Failed health check            | Increase `startPeriod` in deployment spec     |
| Missing ConfigMap/Secret       | Apply configmap.yaml and secret.yaml          |
| Database connection failure    | Ensure PostgreSQL is running and accessible   |

### 3.3 Services not reachable

```bash
kubectl get svc -n vigilantai
kubectl get endpoints -n vigilantai
```

**Check:**

- Service selector matches pod labels
- Ports are correctly mapped
- Network policies allow traffic

### 3.4 Ingress not working

```bash
kubectl get ingress -n vigilantai
kubectl describe ingress <ingress-name> -n vigilantai
```

**Check:**

- Ingress controller is installed
- TLS certificate is valid
- Backend services are healthy

---

## 4. Backend Service Issues

### 4.1 Backend won't start

**Check logs:**

```bash
docker compose logs backend
```

**Common issues:**

| Symptom                        | Cause                             | Solution                           |
|--------------------------------|-----------------------------------|------------------------------------|
| `DATABASE_URL not set`        | Missing env var                   | Set `DATABASE_URL` in `.env`       |
| `JWT_PRIVATE_KEY not set`     | Missing JWT keys                  | Generate keys (see Section 9)      |
| `connection refused`          | PostgreSQL not ready              | Wait for postgres healthcheck      |
| `migration error`             | Schema mismatch                   | Check migration files              |

### 4.2 High API latency

**Diagnosis:**

```promql
# Check P95 latency in Prometheus
histogram_quantile(0.95, rate(vigilantai_http_request_duration_seconds_bucket[5m]))
```

**Causes and solutions:**

| Cause                          | Solution                                      |
|--------------------------------|-----------------------------------------------|
| Slow database queries          | Add indexes; check `vigilantai_db_query_duration_seconds` |
| Connection pool exhaustion     | Increase pool size; check for connection leaks |
| High CPU usage                 | Scale horizontally or vertically               |
| Memory pressure                | Increase container memory limit               |

### 4.3 500 Internal Server Error

**Check:**

```bash
# Check backend logs for stack traces
docker compose logs backend | grep -i error

# Check Prometheus for error rate
rate(vigilantai_http_requests_total{status="500"}[5m])
```

---

## 5. AI Service Issues

### 5.1 Model fails to load

**Symptom:** AI service starts but inference returns errors

**Check logs:**

```bash
docker compose logs ai-service
```

**Common causes:**

| Cause                          | Solution                                      |
|--------------------------------|-----------------------------------------------|
| Model file not found           | Check `AI_SERVICE_DEFAULT_MODEL` value         |
| CUDA not available             | Set `AI_SERVICE_DEVICE=cpu`                    |
| Insufficient GPU memory        | Use smaller model (`yolov8n` vs `yolov8x`)    |
| Network timeout during download| Pre-download model weights                     |

### 5.2 High inference latency

**Diagnosis:**

```promql
histogram_quantile(0.95, rate(vigilantai_ai_inference_latency_seconds_bucket[5m]))
```

**Solutions:**

| Action                          | Impact                                      |
|---------------------------------|----------------------------------------------|
| Switch from CPU to GPU          | 10-50x speedup                              |
| Use smaller model variant       | Faster but less accurate                     |
| Reduce input resolution         | Faster inference, lower accuracy             |
| Enable batch inference          | Better GPU utilization                       |

### 5.3 AI service not responding

```bash
# Check health
curl http://localhost:8081/health

# Check if port is listening
docker compose exec ai-service netstat -tlnp
```

---

## 6. Camera Gateway Issues

### 6.1 Cameras showing offline

**Diagnosis:**

```bash
# Check gateway logs
docker compose logs camera-gateway

# Check Prometheus metrics
vigilantai_gateway_cameras_offline
vigilantai_gateway_reconnect_attempts_total
```

**Common causes:**

| Cause                          | Solution                                      |
|--------------------------------|-----------------------------------------------|
| RTSP URL incorrect             | Verify camera RTSP URL and credentials        |
| Network unreachable            | Check network connectivity to camera subnet   |
| Camera credentials changed     | Update camera configuration in database       |
| Firewall blocking RTSP         | Open port 554 (RTSP) to camera network        |

### 6.2 High reconnect rate

**Symptom:** Many reconnection attempts in logs

**Check:**

```promql
rate(vigilantai_gateway_reconnect_attempts_total[5m])
```

**Solutions:**

- Check network stability
- Increase reconnection backoff parameters
- Verify camera stream is stable

### 6.3 Frame processing bottleneck

**Symptom:** Low frame processing rate

**Check:**

```promql
rate(vigilantai_gateway_frames_processed_total[5m])
```

**Solutions:**

- Reduce FPS on cameras
- Scale gateway horizontally
- Check AI service response time

---

## 7. Dashboard Issues

### 7.1 Dashboard won't start

```bash
docker compose logs dashboard
```

**Common causes:**

| Cause                          | Solution                                      |
|--------------------------------|-----------------------------------------------|
| Backend not healthy            | Dashboard depends on backend healthcheck      |
| Build failure                  | Check Node.js version and npm install         |
| Missing `NEXT_PUBLIC_API_URL`  | Set in build args and environment             |

### 7.2 Dashboard shows "Connection Error"

**Check:**

- Backend is running: `curl http://localhost:8080/api/v1/health`
- `NEXT_PUBLIC_API_URL` is correct in dashboard environment
- CORS is configured (if dashboard is on different origin)
- Browser console for network errors

### 7.3 WebSocket not connecting

**Check:**

- Backend WebSocket endpoint is accessible
- JWT token is valid and not expired
- Browser console for WebSocket errors
- Proxy/load balancer supports WebSocket upgrade

---

## 8. Database Issues

### 8.1 Cannot connect to PostgreSQL

```bash
# Check if postgres is running
docker compose ps postgres

# Check logs
docker compose logs postgres

# Test connection
docker compose exec postgres psql -U vigilant -d vigilantai
```

**Common causes:**

| Cause                          | Solution                                      |
|--------------------------------|-----------------------------------------------|
| PostgreSQL container crashed   | `docker compose restart postgres`             |
| Wrong credentials              | Verify `POSTGRES_USER` and `POSTGRES_PASSWORD` in `.env` |
| Port conflict                  | Change `POSTGRES_PORT` in `.env`              |
| Disk full                     | Clean up disk space: `docker system prune`    |

### 8.2 Slow queries

**Diagnosis:**

```promql
histogram_quantile(0.95, rate(vigilantai_db_query_duration_seconds_bucket[5m]))
```

**Solutions:**

- Add indexes for frequently queried columns
- Optimize query patterns
- Increase connection pool size
- Add read replicas for read-heavy workloads

### 8.3 Connection pool exhaustion

**Symptoms:** High latency, connection timeout errors

**Solutions:**

- Increase `max_connections` in PostgreSQL config
- Increase SQLx pool size in backend config
- Check for connection leaks (unclosed connections)

---

## 9. Redis Issues

### 9.1 Cannot connect to Redis

```bash
# Check if redis is running
docker compose ps redis

# Test connection
docker compose exec redis redis-cli ping
```

**Expected response:** `PONG`

### 9.2 High memory usage

```bash
docker compose exec redis redis-cli info memory
```

**Solutions:**

- Increase `maxmemory` in Redis config
- Check for key proliferation
- Implement TTL on cached keys

### 9.3 Redis data loss after restart

Redis uses AOF persistence by default. If data is lost:

- Check if AOF file exists in the volume
- Verify `appendonly yes` in Redis config
- For cache-only use, data loss is expected and acceptable

---

## 10. JWT / Authentication Issues

### 10.1 "Invalid token" errors

**Causes:**

| Cause                          | Solution                                      |
|--------------------------------|-----------------------------------------------|
| Token expired                  | Client must use refresh token                 |
| Wrong signing key              | Verify `JWT_PUBLIC_KEY` matches `JWT_PRIVATE_KEY` |
| Token tampered                 | Regenerate token                              |
| Clock skew                     | Ensure server time is synchronized (NTP)      |

### 10.2 Cannot generate tokens

**Check:**

```bash
# Verify JWT keys exist in .env
grep JWT_PRIVATE_KEY .env
grep JWT_PUBLIC_KEY .env

# Test key format
echo "$JWT_PRIVATE_KEY" | head -1
# Should show: -----BEGIN PRIVATE KEY-----
```

### 10.3 Refresh token rejected

**Causes:**

- Refresh token was already used (rotation)
- Refresh token expired (7-day limit)
- User was deactivated
- Refresh token not found in database

### 10.4 RBAC permission denied

**Check user roles:**

```sql
SELECT u.email, r.name as role
FROM users u
JOIN user_roles ur ON u.id = ur.user_id
JOIN roles r ON ur.role_id = r.id
WHERE u.id = 'user-uuid';
```

**Check required permission for endpoint** in API documentation.

---

## 11. Monitoring Stack Issues

### 11.1 Prometheus not scraping targets

```bash
# Check Prometheus targets
curl http://localhost:9090/api/v1/targets
```

**Common causes:**

| Cause                          | Solution                                      |
|--------------------------------|-----------------------------------------------|
| Target unreachable             | Check network connectivity                     |
| Wrong metrics path             | Verify `metrics_path` in `prometheus.yml`     |
| Service not exposing metrics   | Ensure `/metrics` endpoint is available        |

### 11.2 Grafana dashboards empty

**Check:**

- Prometheus datasource is configured in Grafana
- Prometheus has data for the selected time range
- Dashboard panels have correct metric queries
- Labels match (check `environment` and `instance` variables)

### 11.3 Loki not receiving logs

```bash
# Check Promtail status
curl http://localhost:9080/ready

# Check Loki status
curl http://localhost:3100/ready
```

**Check Promtail config:**

```bash
docker compose exec promtail cat /etc/promtail/config.yml
```

### 11.4 Alerts not firing

**Check:**

```bash
# Check alert rules in Prometheus
curl http://localhost:9090/api/v1/rules

# Check Alertmanager
curl http://localhost:9093/api/v2/alerts
```

---

## 12. Log Analysis Techniques

### 12.1 Find errors in backend logs

```bash
# Docker Compose
docker compose logs backend 2>&1 | grep -i error

# Kubernetes
kubectl logs deployment/backend -n vigilantai | grep -i error
```

### 12.2 Find slow requests

```bash
# Search for high duration values
docker compose logs backend 2>&1 | grep "duration_ms" | grep -E '"duration_ms":[0-9]{4,}'
```

### 12.3 Find authentication failures

```bash
docker compose logs backend 2>&1 | grep "401\|403\|unauthorized\|forbidden"
```

### 12.4 Trace a specific request

```bash
# Find correlation ID in logs
docker compose logs backend 2>&1 | grep "correlation_id.*<uuid>"
```

### 12.5 Monitor real-time logs

```bash
# Follow all logs
make docker-logs

# Follow specific service
docker compose logs -f backend --tail 100
```

---

## 13. Performance Tuning

### 13.1 Backend Tuning

| Parameter                     | Default | Recommended       | Location                |
|-------------------------------|---------|-------------------|-------------------------|
| Connection pool max           | 20      | 50 (high load)    | Backend config           |
| Tokio worker threads          | auto    | auto (leave as-is)| Tokio runtime config     |
| Request timeout               | 30s     | 30s (adjust as needed) | Tower middleware  |
| Log level                     | info    | warn (production) | `RUST_LOG` env var       |

### 13.2 PostgreSQL Tuning

| Parameter                     | Default | Recommended       | Location                |
|-------------------------------|---------|-------------------|-------------------------|
| `shared_buffers`              | 128MB   | 25% of RAM        | postgresql.conf         |
| `effective_cache_size`        | 4GB     | 75% of RAM        | postgresql.conf         |
| `work_mem`                    | 4MB     | 16-64MB           | postgresql.conf         |
| `max_connections`             | 100     | 200               | postgresql.conf         |
| `checkpoint_completion_target`| 0.5     | 0.9               | postgresql.conf         |

### 13.3 Redis Tuning

| Parameter                     | Default | Recommended       | Location                |
|-------------------------------|---------|-------------------|-------------------------|
| `maxmemory`                   | 2GB     | 4-8GB             | Redis config            |
| `maxmemory-policy`            | allkeys-lru | allkeys-lru   | Redis config            |
| `appendonly`                  | yes     | yes               | Redis config            |

### 13.4 Docker Resource Limits

| Service         | CPU Limit | Memory Limit | Notes                    |
|-----------------|-----------|-------------|--------------------------|
| backend         | 4 cores   | 4 GB        | Increase for high load   |
| ai-service      | 8 cores   | 16 GB       | GPU passthrough if CUDA  |
| camera-gateway  | 4 cores   | 4 GB        | Scale horizontally       |
| postgres        | 4 cores   | 16 GB       | Dedicated node preferred |
| redis           | 2 cores   | 4 GB        | In-memory performance    |
| prometheus      | 2 cores   | 4 GB        | 30-day retention         |
| grafana         | 1 core    | 2 GB        | Light resource usage     |
| loki            | 2 cores   | 4 GB        | Depends on log volume    |
