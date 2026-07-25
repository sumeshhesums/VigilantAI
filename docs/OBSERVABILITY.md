# VigilantAI — Observability Guide

> **Enterprise Security Intelligence Platform**
> Observability Guide — v1.0

---

## Table of Contents

| Section | Title                                |
|---------|--------------------------------------|
| 1       | Overview                             |
| 2       | Metrics (Prometheus)                 |
| 3       | Logs (Loki + Promtail)               |
| 4       | Dashboards (Grafana)                 |
| 5       | Alert Rules                          |
| 6       | Recording Rules                      |
| 7       | Label Conventions                    |
| 8       | Common Queries                       |

---

## 1. Overview

VigilantAI implements the three pillars of observability:

| Pillar   | Tool              | Purpose                                        |
|----------|-------------------|------------------------------------------------|
| Metrics  | Prometheus        | Numerical measurements of system behavior      |
| Logs     | Loki + Promtail   | Discrete events for debugging and auditing     |
| Traces   | (Planned: Tempo)  | Request lifecycle across services              |

**Stack Components:**

| Component     | Version     | Port  | Purpose                          |
|---------------|-------------|-------|----------------------------------|
| Prometheus    | v2.54.1     | 9090  | Metrics collection & storage     |
| Grafana       | 11.2.2      | 3001  | Visualization & dashboards       |
| Loki          | 3.2.1       | 3100  | Log aggregation                  |
| Promtail      | 3.2.1       | 9080  | Log collection from containers   |
| Alertmanager  | v0.27       | 9093  | Alert routing & notification     |

---

## 2. Metrics (Prometheus)

### 2.1 Backend Metrics (10 metrics)

| Metric Name                                      | Type      | Description                                   |
|--------------------------------------------------|-----------|-----------------------------------------------|
| `vigilantai_http_requests_total`                 | Counter   | Total HTTP requests by method, endpoint, status|
| `vigilantai_http_request_duration_seconds`       | Histogram | HTTP request latency distribution              |
| `vigilantai_http_requests_in_flight`             | Gauge     | Currently in-flight HTTP requests              |
| `vigilantai_jwt_auth_success_total`              | Counter   | Successful JWT authentication attempts         |
| `vigilantai_jwt_auth_failure_total`              | Counter   | Failed JWT authentication attempts             |
| `vigilantai_rbac_denied_total`                   | Counter   | RBAC authorization denials                     |
| `vigilantai_incidents_created_total`             | Counter   | Total incidents created                        |
| `vigilantai_evidence_uploads_total`              | Counter   | Total evidence files uploaded                  |
| `vigilantai_notifications_sent_total`            | Counter   | Total notifications sent                       |
| `vigilantai_db_query_duration_seconds`           | Histogram | Database query latency distribution            |

### 2.2 Camera Gateway Metrics (9 metrics)

| Metric Name                                      | Type      | Description                                   |
|--------------------------------------------------|-----------|-----------------------------------------------|
| `vigilantai_gateway_cameras_connected`           | Gauge     | Currently connected cameras                    |
| `vigilantai_gateway_cameras_online`              | Gauge     | Cameras streaming successfully                 |
| `vigilantai_gateway_cameras_offline`             | Gauge     | Cameras with failed connections                |
| `vigilantai_gateway_frames_processed_total`      | Counter   | Total frames extracted from RTSP streams       |
| `vigilantai_gateway_ai_requests_total`           | Counter   | Total requests sent to AI service              |
| `vigilantai_gateway_ai_failures_total`           | Counter   | Failed requests to AI service                  |
| `vigilantai_gateway_backend_publishes_total`     | Counter   | Successful publishes to backend                |
| `vigilantai_gateway_backend_publish_failures_total` | Counter | Failed publishes to backend                   |
| `vigilantai_gateway_reconnect_attempts_total`    | Counter   | Camera reconnection attempts                   |

### 2.3 AI Service Metrics

| Metric Name                                      | Type      | Description                                   |
|--------------------------------------------------|-----------|-----------------------------------------------|
| `vigilantai_ai_inference_requests_total`         | Counter   | Total inference requests received              |
| `vigilantai_ai_inference_failures_total`         | Counter   | Failed inference requests                      |
| `vigilantai_ai_inference_latency_seconds`        | Histogram | Inference latency distribution                 |
| `vigilantai_ai_model_load_time_seconds`          | Gauge     | Time to load model weights                     |
| `vigilantai_ai_images_processed_total`           | Counter   | Total images processed                         |
| `vigilantai_ai_detections_total`                 | Counter   | Total detections produced                      |
| `vigilantai_ai_detection_confidence`             | Histogram | Detection confidence score distribution        |
| `vigilantai_ai_detections_per_image`             | Histogram | Number of detections per input image           |
| `vigilantai_ai_cpu_usage_percent`                | Gauge     | CPU usage percentage                           |
| `vigilantai_ai_memory_usage_bytes`               | Gauge     | Memory usage in bytes                          |

### 2.4 Dashboard Metrics

| Metric Name                                      | Type      | Description                                   |
|--------------------------------------------------|-----------|-----------------------------------------------|
| `vigilantai_dashboard_up`                        | Gauge     | Dashboard service availability (1 = up)       |
| `vigilantai_dashboard_uptime_seconds`            | Gauge     | Dashboard uptime in seconds                    |
| `vigilantai_dashboard_build_info`                | Gauge     | Build version information                      |

---

## 3. Logs (Loki + Promtail)

### 3.1 Log Pipeline

```
Containers ──► Promtail ──► Loki ──► Grafana
(host logs)     (collection)  (storage)  (query)
```

### 3.2 Log Collection Sources

| Source                    | Collection Method               | Labels                     |
|---------------------------|----------------------------------|-----------------------------|
| Docker container logs     | Docker socket + log driver       | `container`, `service`      |
| Application stdout/stderr | Container log capture            | `service`, `level`          |
| Host system logs          | `/var/log` mount                 | `job`, `filename`           |
| Prometheus logs           | `/var/lib/docker/containers`     | `container`                 |

### 3.3 Log Label Schema

| Label           | Example                  | Description                    |
|-----------------|--------------------------|--------------------------------|
| `service`       | `backend`                | Service name                   |
| `environment`   | `production`             | Deployment environment         |
| `level`         | `info`, `warn`, `error`  | Log level                      |
| `container`     | `vigilantai-backend-1`   | Container name                 |
| `instance`      | `backend:8080`           | Service instance               |

### 3.4 Structured Log Format

All VigilantAI services emit JSON-structured logs:

```json
{
  "timestamp": "2026-07-22T22:15:05.123Z",
  "level": "info",
  "service": "backend",
  "message": "Incident created",
  "incident_id": "inc-uuid-1",
  "user_id": "user-uuid-1",
  "camera_id": "cam-uuid-1",
  "correlation_id": "req-uuid-1",
  "duration_ms": 45
}
```

---

## 4. Dashboards (Grafana)

VigilantAI ships with 5 pre-provisioned Grafana dashboards.

**Grafana URL:** `http://localhost:3001` (default credentials: `admin` / `admin`)

### 4.1 Platform Overview Dashboard

**UID:** `vigilantai-platform-overview`
**Purpose:** High-level system health across all VigilantAI services

| Row              | Panel                      | Description                          |
|------------------|----------------------------|--------------------------------------|
| System Health    | Backend Status             | Up/down status of backend service    |
|                  | AI Service Status          | Up/down status of AI service         |
|                  | Camera Gateway Status      | Up/down status of camera gateway     |
|                  | Dashboard Status           | Up/down status of dashboard          |
|                  | PostgreSQL Status          | Database reachability                |
|                  | Redis Status               | Cache reachability                   |
| Request Metrics  | Total Request Rate         | Requests per second across backend   |
|                  | Error Rate                 | 5xx error rate percentage            |
|                  | Average Latency P95        | 95th percentile request latency      |
| Business Metrics | Incidents Created          | Incident creation rate               |
|                  | Evidence Uploaded          | Evidence upload rate                 |
|                  | Notifications Sent         | Notification dispatch rate           |
|                  | Active Connections         | WebSocket connections count          |
| Camera Fleet     | Cameras Online             | Online camera count                  |
|                  | Camera Availability Ratio  | Online / total camera percentage     |
|                  | AI Inference Rate          | Inferences per second                |
| Alerts           | Active Alerts              | Current active alert count           |
| Logs             | Recent Logs                | Live log stream from all services    |

### 4.2 Backend API Dashboard

**Purpose:** Detailed HTTP, authentication, business, and database metrics

| Row                | Panel                          | Description                        |
|--------------------|--------------------------------|------------------------------------|
| HTTP Metrics       | Request Rate by Endpoint       | Requests/s grouped by endpoint     |
|                    | Request Rate by Status         | Requests/s grouped by status code  |
|                    | Latency P50/P95/P99            | Request latency percentiles        |
|                    | Active Connections             | Current in-flight requests         |
| Authentication     | JWT Auth Success Rate          | Successful JWT validations         |
|                    | JWT Auth Failure Rate          | Failed JWT validations             |
|                    | RBAC Authorization Failures    | Permission denied count            |
| Business Metrics   | Incidents Created Rate         | New incidents per minute           |
|                    | Evidence Upload Rate           | Evidence uploads per minute        |
|                    | Notification Rate              | Notifications sent per minute      |
|                    | Total Incidents                | Cumulative incident count          |
| Database           | Query Duration P95             | 95th percentile DB query latency   |
|                    | Query Duration Heatmap         | Query latency distribution heatmap |
| Logs               | Backend Logs                   | Live log stream                    |
|                    | Error Logs                     | Error-level log entries            |

### 4.3 AI Service Dashboard

**Purpose:** Inference performance, detection metrics, and resource usage

| Row               | Panel                              | Description                      |
|-------------------|------------------------------------|----------------------------------|
| Inference Overview| Inference Rate                     | Inferences per second            |
|                   | Inference Failures                 | Failed inference count           |
|                   | Failure Rate                       | Failure percentage               |
|                   | Total Inferences                   | Cumulative inference count       |
| Latency           | Inference Latency P50/P95/P99     | Latency percentiles              |
|                   | Inference Latency Histogram        | Latency distribution             |
|                   | Model Load Time                    | Time to load model weights       |
| Detections        | Images Processed Rate              | Images processed per second      |
|                   | Detections Rate                    | Detections per second            |
|                   | Detection Confidence Distribution  | Confidence score histogram       |
|                   | Detections Per Image               | Average detections per frame     |
| Resources         | CPU Usage                          | CPU utilization percentage       |
|                   | Memory Usage                       | Memory utilization               |
| Logs              | AI Service Logs                    | Live log stream                  |
|                   | Inference Errors                   | Error log entries                |

### 4.4 Camera Gateway Dashboard

**Purpose:** Camera fleet status, frame processing, and backend publishing

| Row                | Panel                        | Description                        |
|--------------------|------------------------------|------------------------------------|
| Camera Fleet Status| Cameras Connected            | Connected camera gauge             |
|                    | Cameras Online               | Online camera gauge                |
|                    | Cameras Offline              | Offline camera gauge               |
|                    | Camera Availability Ratio    | Online/total percentage            |
| Camera Fleet Timeline | Camera Status Over Time   | Camera status stacked chart        |
|                    | Reconnect Attempts           | Reconnection attempt rate          |
| Processing         | Frames Processed Rate        | Frames extracted per second        |
|                    | AI Requests Rate             | Requests to AI service per second  |
|                    | AI Failures Rate             | Failed AI requests per second      |
| Backend Publishing | Backend Publish Rate         | Events published to backend/s      |
|                    | Backend Publish Failures     | Failed backend publishes           |
|                    | Publish Success Ratio        | Successful publish percentage      |
| Logs               | Gateway Logs                 | Live log stream                    |
|                    | Connection Errors            | Connection error log entries       |

### 4.5 Dashboard Monitoring Dashboard

**Purpose:** Health and availability of the Next.js dashboard service

| Row            | Panel                        | Description                        |
|----------------|------------------------------|------------------------------------|
| Availability   | Dashboard Status             | Up/down status (stat panel)        |
|                | Uptime                       | Uptime percentage                  |
|                | Build Version                | Current build version              |
| Service Health | Dashboard Up Over Time       | Availability timeline              |
|                | Uptime Over Time             | Uptime percentage over time        |
| Alert Status   | Active Alerts for Dashboard  | Dashboard-related active alerts    |

---

## 5. Alert Rules

VigilantAI defines 12 alert rules in `monitoring/prometheus/alert.rules.yml`:

### 5.1 Service Availability Alerts

| Alert                | Expression                              | Duration | Severity | Description                        |
|----------------------|-----------------------------------------|----------|----------|------------------------------------|
| `BackendDown`        | `up{job="vigilantai-backend"} == 0`    | 1 min    | critical | Backend unreachable for 1+ minute  |
| `AiServiceDown`      | `up{job="vigilantai-ai-service"} == 0` | 1 min    | critical | AI service unreachable for 1+ minute |
| `GatewayDown`        | `up{job="vigilantai-camera-gateway"} == 0` | 1 min | critical | Gateway unreachable for 1+ minute |
| `DashboardDown`      | `up{job="vigilantai-dashboard"} == 0`  | 2 min    | warning  | Dashboard unreachable for 2+ minutes |
| `PostgresUnreachable`| `up{job="vigilantai-postgres"} == 0`   | 1 min    | critical | PostgreSQL unreachable for 1+ minute |
| `RedisUnreachable`   | `up{job="vigilantai-redis"} == 0`      | 1 min    | critical | Redis unreachable for 1+ minute    |

### 5.2 Performance Alerts

| Alert                    | Expression                                                      | Duration | Severity | Description                              |
|--------------------------|-----------------------------------------------------------------|----------|----------|------------------------------------------|
| `HighBackendLatency`     | `histogram_quantile(0.95, rate(vigilantai_http_request_duration_seconds_bucket[5m])) > 2.0` | 5 min | warning | P95 latency > 2s |
| `HighAiInferenceLatency` | `histogram_quantile(0.95, rate(vigilantai_ai_inference_latency_seconds_bucket[5m])) > 5.0` | 5 min | warning | P95 AI latency > 5s |
| `HighDbQueryLatency`     | `histogram_quantile(0.95, rate(vigilantai_db_query_duration_seconds_bucket[5m])) > 1.0` | 5 min | warning | P95 DB latency > 1s |

### 5.3 Error Rate Alerts

| Alert                    | Expression                                                      | Duration | Severity | Description                              |
|--------------------------|-----------------------------------------------------------------|----------|----------|------------------------------------------|
| `HighBackendErrorRate`   | `rate(vigilantai_http_requests_total{status=~"5.."}[5m]) / rate(vigilantai_http_requests_total[5m]) > 0.05` | 5 min | warning | >5% 5xx error rate |
| `HighAiFailureRate`      | `rate(vigilantai_ai_inference_failures_total[5m]) / rate(vigilantai_ai_inference_requests_total[5m]) > 0.10` | 5 min | warning | >10% AI failure rate |

### 5.4 Operational Alerts

| Alert               | Expression                                  | Duration | Severity | Description                            |
|---------------------|---------------------------------------------|----------|----------|----------------------------------------|
| `CamerasOffline`    | `vigilantai_gateway_cameras_offline > 0`    | 5 min    | warning  | One or more cameras offline            |

---

## 6. Recording Rules

Pre-computed recordings in `monitoring/prometheus/recording.rules.yml`:

| Recording Rule                             | Expression                                             | Purpose                      |
|--------------------------------------------|--------------------------------------------------------|------------------------------|
| `vigilantai:http_requests:rate5m`          | `rate(vigilantai_http_requests_total[5m])`             | Request rate (5m)            |
| `vigilantai:http_request_duration:avg5m`   | Avg request duration (5m)                              | Average latency              |
| `vigilantai:http_request_duration:p95_5m`  | P95 request duration (5m)                              | P95 latency                  |
| `vigilantai:ai_inference_latency:avg5m`    | Avg AI inference latency (5m)                          | Average AI latency           |
| `vigilantai:ai_inference_latency:p95_5m`   | P95 AI inference latency (5m)                          | P95 AI latency               |
| `vigilantai:ai_inference_requests:rate5m`  | `rate(vigilantai_ai_inference_requests_total[5m])`     | Inference request rate       |
| `vigilantai:incidents_created:rate5m`      | `rate(vigilantai_incidents_created_total[5m])`         | Incident creation rate       |
| `vigilantai:evidence_uploads:rate5m`       | `rate(vigilantai_evidence_uploads_total[5m])`          | Evidence upload rate         |
| `vigilantai:notifications_sent:rate5m`     | `rate(vigilantai_notifications_sent_total[5m])`        | Notification send rate       |
| `vigilantai:gateway_cameras_online:ratio`  | `cameras_online / (cameras_online + cameras_offline)`  | Camera availability ratio   |
| `vigilantai:ai_failure_rate:ratio5m`       | `rate(ai_failures[5m]) / rate(ai_requests[5m])`       | AI failure rate ratio        |
| `vigilantai:backend_publishes:rate5m`      | `rate(vigilantai_gateway_backend_publishes_total[5m])` | Backend publish rate         |

---

## 7. Label Conventions

### 7.1 Required Labels

| Label           | Values                              | Description                    |
|-----------------|-------------------------------------|--------------------------------|
| `service`       | `backend`, `ai-service`, `camera-gateway`, `dashboard` | Service identifier |
| `environment`   | `development`, `staging`, `production` | Deployment environment     |
| `job`           | `vigilantai-backend`, `vigilantai-ai-service`, etc. | Prometheus job name |

### 7.2 Optional Labels

| Label           | Values                              | Description                    |
|-----------------|-------------------------------------|--------------------------------|
| `instance`      | `backend:8080`, `ai-service:8081`  | Service instance endpoint      |
| `method`        | `GET`, `POST`, `PATCH`, `DELETE`   | HTTP method                    |
| `endpoint`      | `/api/v1/cameras`, etc.            | API endpoint                   |
| `status`        | `200`, `401`, `500`, etc.          | HTTP status code               |
| `severity`      | `critical`, `warning`, `info`      | Alert severity level           |

### 7.3 Naming Rules

- Use snake_case for all label names
- Use lowercase for all label values
- Avoid high-cardinality labels (no user IDs, UUIDs, or free-text in labels)
- Maximum 10 labels per metric series

---

## 8. Common Queries

### 8.1 PromQL Queries

**Request rate (per second):**

```promql
rate(vigilantai_http_requests_total[5m])
```

**P95 request latency:**

```promql
histogram_quantile(0.95, rate(vigilantai_http_request_duration_seconds_bucket[5m]))
```

**Error rate (percentage):**

```promql
rate(vigilantai_http_requests_total{status=~"5.."}[5m]) / rate(vigilantai_http_requests_total[5m]) * 100
```

**Camera availability:**

```promql
vigilantai_gateway_cameras_online / (vigilantai_gateway_cameras_online + vigilantai_gateway_cameras_offline) * 100
```

**AI inference rate:**

```promql
rate(vigilantai_ai_inference_requests_total[5m])
```

**AI failure rate:**

```promql
rate(vigilantai_ai_inference_failures_total[5m]) / rate(vigilantai_ai_inference_requests_total[5m]) * 100
```

**Incidents created per minute:**

```promql
rate(vigilantai_incidents_created_total[5m]) * 60
```

**Database query P95 latency:**

```promql
histogram_quantile(0.95, rate(vigilantai_db_query_duration_seconds_bucket[5m]))
```

**JWT authentication failure rate:**

```promql
rate(vigilantai_jwt_auth_failure_total[5m]) * 60
```

### 8.2 LogQL Queries

**Backend error logs:**

```logql
{service="backend"} | json | level="error"
```

**AI service logs:**

```logql
{service="ai-service"} | json
```

**Camera gateway connection errors:**

```logql
{service="camera-gateway"} | json | message=~ ".*connection.*error.*"
```

**All error logs across services:**

```logql
{service=~"backend|ai-service|camera-gateway|dashboard"} | json | level="error"
```

**Logs with specific correlation ID:**

```logql
{service="backend"} | json | correlation_id="req-uuid-1"
```

**Request logs with high latency (> 1s):**

```logql
{service="backend"} | json | duration_ms > 1000
```

**Recent errors (last 15 minutes):**

```logql
{service=~".*"} | json | level="error" | line_format "{{.timestamp}} [{{.service}}] {{.message}}"
```
