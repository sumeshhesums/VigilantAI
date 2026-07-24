# VigilantAI Prometheus Monitoring

## Architecture

```
Prometheus Server (:9090)
    |
    |-- scrape --> backend (:8080/metrics)
    |-- scrape --> camera-gateway (:8082/metrics)
    |-- scrape --> ai-service (:8081/metrics)
    |-- scrape --> dashboard (:3000/api/metrics)
    |-- scrape --> postgres-exporter (:9187/metrics) [placeholder]
    |-- scrape --> redis-exporter (:9121/metrics) [placeholder]
    |
    |-- evaluate --> alert.rules.yml
    |-- evaluate --> recording.rules.yml
```

## How Scraping Works

Prometheus pulls metrics from each service's `/metrics` endpoint at a 15-second interval. Each service exposes metrics in the Prometheus text exposition format (OpenMetrics-compatible).

### Scrape Targets

| Service | Endpoint | Port | Interval |
|---------|----------|------|----------|
| Backend | `/metrics` | 8080 | 15s |
| Camera Gateway | `/metrics` | 8082 | 15s |
| AI Service | `/metrics` | 8081 | 15s |
| Dashboard | `/api/metrics` | 3000 | 15s |
| PostgreSQL Exporter | `/metrics` | 9187 | 15s |
| Redis Exporter | `/metrics` | 9121 | 15s |

## Metric Naming

All application metrics use the `vigilantai_` prefix:

- `vigilantai_http_requests_total` - Total HTTP requests
- `vigilantai_http_request_duration_seconds` - Request latency histogram
- `vigilantai_jwt_auth_success_total` - JWT auth successes
- `vigilantai_jwt_auth_failure_total` - JWT auth failures
- `vigilantai_rbac_authorization_failures_total` - RBAC failures
- `vigilantai_incidents_created_total` - Incidents created
- `vigilantai_evidence_uploads_total` - Evidence uploads
- `vigilantai_notifications_sent_total` - Notifications sent
- `vigilantai_db_query_duration_seconds` - DB query latency
- `vigilantai_active_connections` - Active connections
- `vigilantai_gateway_cameras_connected` - Connected cameras
- `vigilantai_gateway_cameras_online` - Online cameras
- `vigilantai_gateway_cameras_offline` - Offline cameras
- `vigilantai_gateway_reconnect_attempts_total` - Reconnect attempts
- `vigilantai_gateway_frames_processed_total` - Frames processed
- `vigilantai_gateway_ai_requests_total` - AI requests from gateway
- `vigilantai_gateway_ai_failures_total` - AI failures from gateway
- `vigilantai_gateway_backend_publishes_total` - Backend publishes
- `vigilantai_gateway_backend_publish_failures_total` - Publish failures
- `vigilantai_ai_inference_requests_total` - Inference requests
- `vigilantai_ai_inference_failures_total` - Inference failures
- `vigilantai_ai_inference_latency_seconds` - Inference latency
- `vigilantai_ai_images_processed_total` - Images processed
- `vigilantai_ai_detections_total` - Detections produced
- `vigilantai_ai_detection_confidence` - Confidence distribution
- `vigilantai_ai_model_load_time_seconds` - Model load time
- `vigilantai_ai_cpu_usage_percent` - CPU usage
- `vigilantai_ai_memory_usage_bytes` - Memory usage
- `vigilantai_dashboard_up` - Dashboard up
- `vigilantai_dashboard_uptime_seconds` - Dashboard uptime

## Alert Rules

| Alert | Condition | Severity |
|-------|-----------|----------|
| BackendDown | `up{job="vigilantai-backend"} == 0` for 1m | critical |
| AiServiceDown | `up{job="vigilantai-ai-service"} == 0` for 1m | critical |
| GatewayDown | `up{job="vigilantai-camera-gateway"} == 0` for 1m | critical |
| DashboardDown | `up{job="vigilantai-dashboard"} == 0` for 2m | warning |
| HighBackendLatency | P95 > 2s for 5m | warning |
| HighAiInferenceLatency | P95 > 5s for 5m | warning |
| HighBackendErrorRate | 5xx > 5% for 5m | warning |
| HighAiFailureRate | Failures > 10% for 5m | warning |
| CamerasOffline | Any cameras offline for 5m | warning |
| PostgresUnreachable | `up{job="vigilantai-postgres"} == 0` for 1m | critical |
| HighDbQueryLatency | P95 > 1s for 5m | warning |
| RedisUnreachable | `up{job="vigilantai-redis"} == 0` for 1m | critical |

## Recording Rules

Pre-computed metrics for dashboards and alerts:

- `vigilantai:http_requests:rate5m` - Request rate
- `vigilantai:http_request_duration:avg5m` - Average request duration
- `vigilantai:ai_inference_latency:avg5m` - Average inference latency
- `vigilantai:ai_inference_requests:rate5m` - Inference request rate
- `vigilantai:incidents_created:rate5m` - Incident creation rate
- `vigilantai:evidence_uploads:rate5m` - Evidence upload rate
- `vigilantai:notifications_sent:rate5m` - Notification rate
- `vigilantai:gateway_cameras_online:ratio` - Camera availability ratio
- `vigilantai:ai_failure_rate:ratio5m` - AI failure rate ratio
- `vigilantai:backend_publishes:rate5m` - Backend publish rate

## Docker Compose

Prometheus is included in `docker-compose.yml`:

```bash
docker compose up -d prometheus
```

Access Prometheus UI at http://localhost:9090

## Kubernetes

Prometheus annotations are added to all K8s deployments for pod-level scraping.

## Future Grafana Integration

To add Grafana dashboards:

1. Add Grafana service to docker-compose
2. Configure Prometheus as a data source
3. Import dashboards using the `vigilantai_` metric prefix
4. Key panels: Request rate, Error rate, Latency percentiles, Camera status, AI inference metrics
