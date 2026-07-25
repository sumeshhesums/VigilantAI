# VigilantAI Grafana Dashboards

Production-grade Grafana dashboards for the VigilantAI security platform with auto-provisioned Prometheus datasource.

## Dashboards

| Dashboard | UID | Description |
|-----------|-----|-------------|
| Platform Overview | `vigilantai-platform-overview` | System health, request metrics, business KPIs, camera fleet |
| Backend API | `vigilantai-backend` | HTTP metrics, authentication, RBAC, business metrics, database |
| Camera Gateway | `vigilantai-camera-gateway` | Camera fleet status, AI requests, backend publishing |
| AI Service | `vigilantai-ai-service` | Inference metrics, latency, detections, resource usage |
| Dashboard Monitoring | `vigilantai-dashboard-monitoring` | Next.js dashboard health and uptime |

## Quick Start

### Docker Compose

```bash
docker compose up -d grafana
```

Grafana available at `http://localhost:3001` (default credentials: `admin`/`admin`).

### Kubernetes

```bash
kubectl apply -k k8s/grafana/
```

Or with kustomize:

```bash
cd k8s/grafana && kustomize build . | kubectl apply -f -
```

## Configuration

### Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `GRAFANA_PORT` | `3001` | Host port for Grafana |
| `GF_SECURITY_ADMIN_USER` | `admin` | Admin username |
| `GF_SECURITY_ADMIN_PASSWORD` | `admin` | Admin password |
| `GF_SERVER_ROOT_URL` | `http://localhost:3001` | Public URL |
| `GF_USERS_ALLOW_SIGN_UP` | `false` | Allow self-registration |

### Auto-Provisioned Resources

- **Datasource**: Prometheus at `http://prometheus:9090`
- **Dashboards**: All 5 dashboards in `VigilantAI` folder
- **Home Dashboard**: Platform Overview

## Directory Structure

```
monitoring/grafana/
  provisioning/
    datasources/
      prometheus.yml       # Prometheus datasource config
    dashboards/
      dashboard.yml        # Dashboard provisioning config
  dashboards/
    platform-overview.json # Platform overview (17 panels)
    backend.json           # Backend API (17 panels)
    camera-gateway.json    # Camera gateway (12 panels)
    ai-service.json        # AI service (13 panels)
    dashboard-monitoring.json # Dashboard monitoring (6 panels)
  README.md
```

## K8s Manifests

```
k8s/grafana/
  configmap.yaml          # Grafana config, datasource, provisioning
  deployment.yaml         # Grafana deployment with health checks
  service.yaml            # ClusterIP service
  secret.yaml             # Admin credentials (change in production)
  kustomization.yaml      # Kustomize config for dashboard ConfigMap generation
```

## Customization

### Adding Dashboards

1. Place the JSON file in `monitoring/grafana/dashboards/`
2. The dashboard provider auto-discovers changes every 30 seconds
3. For K8s, regenerate the ConfigMap: `kubectl create configmap grafana-dashboards --from-file=monitoring/grafana/dashboards/`

### Changing Defaults

Edit `monitoring/grafana/provisioning/datasources/prometheus.yml` to update the Prometheus URL or add additional datasources.

## Access

- **Docker Compose**: `http://localhost:3001`
- **Kubernetes**: `kubectl port-forward svc/grafana 3001:3000 -n vigilantai`
