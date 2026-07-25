# VigilantAI Centralized Logging (Loki + Promtail)

Production-grade centralized logging using Grafana Loki for log aggregation and Promtail for log collection.

## Architecture

```
┌──────────────┐   ┌──────────────┐   ┌──────────────┐
│   backend    │   │ camera-gw    │   │  ai-service  │
│  (Docker/K8s)│   │ (Docker/K8s) │   │ (Docker/K8s) │
└──────┬───────┘   └──────┬───────┘   └──────┬───────┘
       │                  │                  │
       └──────────────────┼──────────────────┘
                          │
                   ┌──────▼──────┐
                   │   Promtail  │
                   │  (agent)    │
                   └──────┬──────┘
                          │ push
                   ┌──────▼──────┐
                   │    Loki     │
                   │  (storage)  │
                   └──────┬──────┘
                          │ query
                   ┌──────▼──────┐
                   │   Grafana   │
                   │ (dashboards)│
                   └─────────────┘
```

### Components

| Component | Port | Description |
|-----------|------|-------------|
| Loki | 3100 | Log aggregation system |
| Promtail | 9080 | Log collection agent |

## Labels

All logs are labeled with consistent metadata:

| Label | Description | Example |
|-------|-------------|---------|
| `service` | Application name | `backend`, `camera-gateway`, `ai-service`, `dashboard` |
| `namespace` | K8s namespace | `vigilantai` |
| `pod` | K8s pod name | `backend-abc123` |
| `container` | Container name | `backend` |
| `level` | Log level | `info`, `warn`, `error`, `debug` |
| `environment` | Deployment env | `production`, `staging`, `development` |
| `component` | K8s component | `monitoring`, `api` |
| `part_of` | K8s part-of | `vigilantai` |

## Log Queries

### Common LogQL Queries

```logql
# All logs for a service
{service="backend"}

# Errors only
{service="backend"} | level="error"

# All errors across services
{service=~"backend|camera-gateway|ai-service"} | level="error"

# Logs containing specific text
{service="backend"} |~ "(?i)(error|exception|panic)"

# Exclude health checks
{service="backend"} !~ "health"

# JSON parsing
{service="backend"} | json | level="error"

# Filter by environment
{service="backend", environment="production"} | level="error"

# Rate of errors
rate({service="backend"} | level="error" [5m])

# Top error messages
topk(10, sum by (message)(count_over_time({service="backend"} | level="error" [1h])))
```

## Grafana Integration

### Auto-Provisioned Datasource

Loki is automatically provisioned as a Grafana datasource:
- **Docker Compose**: `monitoring/grafana/provisioning/datasources/prometheus.yml`
- **Kubernetes**: `k8s/grafana/configmap.yaml`

### Dashboard Log Panels

Each dashboard includes log panels:

| Dashboard | Log Panels |
|-----------|------------|
| Platform Overview | Recent Logs (all services) |
| Backend API | Backend Logs + Error Logs |
| Camera Gateway | Gateway Logs + Connection Errors |
| AI Service | AI Service Logs + Inference Errors |

### Accessing Logs

1. Open Grafana (`http://localhost:3001`)
2. Navigate to **Explore** -> select **Loki** datasource
3. Or use the log panels in any dashboard

## Retention

| Setting | Value |
|---------|-------|
| `reject_old_samples_max_age` | 7 days (168h) |
| `compactor.retention_enabled` | true |
| `compactor.retention_delete_delay` | 2 hours |
| `compactor.compaction_interval` | 10 minutes |

To change retention, edit `monitoring/loki/loki-config.yml`:

```yaml
limits_config:
  reject_old_samples_max_age: 336h  # 14 days
```

## Docker Compose

```bash
# Start logging stack
docker compose up -d loki promtail

# View logs
docker compose logs -f loki
docker compose logs -f promtail

# Check Loki health
curl http://localhost:3100/ready
```

## Kubernetes

```bash
# Deploy logging stack
kubectl apply -k k8s/loki/
kubectl apply -k k8s/promtail/

# Check pods
kubectl get pods -n vigilantai -l app.kubernetes.io/name=loki
kubectl get pods -n vigilantai -l app.kubernetes.io/name=promtail

# Port-forward Loki
kubectl port-forward svc/loki 3100:3100 -n vigilantai
```

## Troubleshooting

### Promtail not collecting logs

```bash
# Check Promtail targets
curl http://localhost:9080/targets

# Check Promtail config
curl http://localhost:9080/config
```

### Loki not receiving logs

```bash
# Check Loki ready
curl http://localhost:3100/ready

# Check Loki labels
curl http://localhost:3100/loki/api/v1/labels

# Check specific label values
curl http://localhost:3100/loki/api/v1/label/service/values
```

### Common Issues

1. **Docker socket not accessible**: Ensure `/var/run/docker.sock` is mounted
2. **No logs in Loki**: Check Promtail is running and targeting correct containers
3. **High memory usage**: Reduce `max_entries_limit_per_query` in Loki config
4. **Logs not labeled**: Ensure pipeline_stages extract level from JSON output

## File Structure

```
monitoring/loki/
  loki-config.yml           # Loki server configuration
  promtail-config.yml       # Promtail scrape configuration
  README.md

k8s/loki/
  configmap.yaml            # Loki configuration
  deployment.yaml           # Loki deployment
  service.yaml              # Loki service
  kustomization.yaml        # Kustomize config

k8s/promtail/
  configmap.yaml            # Promtail configuration
  rbac.yaml                 # ServiceAccount, ClusterRole, ClusterRoleBinding
  daemonset.yaml            # Promtail daemonset
  kustomization.yaml        # Kustomize config
```
