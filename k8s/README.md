# VigilantAI Kubernetes Deployment

## Architecture

```
Ingress (nginx)
    |
    v
dashboard (ClusterIP :3000)
    |
    v
backend (ClusterIP :8080) -----> postgres (ClusterIP :5432)
    |                             redis (ClusterIP :6379)
    v
camera-gateway (ClusterIP :8082)
    |
    v
ai-service (ClusterIP :8081)
```

## Prerequisites

- Kubernetes cluster (1.25+)
- kubectl configured
- Container images built and pushed to a registry
- nginx ingress controller installed

## Directory Structure

```
k8s/
├── namespace.yaml
├── configmap.yaml
├── secret.example.yaml
├── persistent-volume.yaml
├── persistent-volume-claim.yaml
├── ingress.yaml
├── README.md
├── dashboard/
│   ├── deployment.yaml
│   └── service.yaml
├── backend/
│   ├── deployment.yaml
│   └── service.yaml
├── ai-service/
│   ├── deployment.yaml
│   └── service.yaml
├── camera-gateway/
│   ├── deployment.yaml
│   └── service.yaml
├── postgres/
│   ├── deployment.yaml
│   └── service.yaml
├── redis/
│   ├── deployment.yaml
│   └── service.yaml
└── hpa/
    ├── backend-hpa.yaml
    ├── ai-service-hpa.yaml
    └── camera-gateway-hpa.yaml
```

## Deployment Order

### 1. Create secrets (REQUIRED)

```bash
cp secret.example.yaml secret.yaml
# Edit secret.yaml with real values
# Generate JWT keys:
openssl genrsa 4096 | openssl pkcs8 -topk8 -nocrypt -outform PEM > jwt_private.pem
openssl rsa -in jwt_private.pem -pubout > jwt_public.pem
```

### 2. Apply all manifests

```bash
# Apply base resources
kubectl apply -f k8s/namespace.yaml
kubectl apply -f k8s/configmap.yaml
kubectl apply -f k8s/secret.yaml

# Apply storage
kubectl apply -f k8s/persistent-volume.yaml
kubectl apply -f k8s/persistent-volume-claim.yaml

# Apply infrastructure services
kubectl apply -f k8s/postgres/
kubectl apply -f k8s/redis/

# Wait for infrastructure to be ready
kubectl -n vigilantai wait --for=condition=available deployment/postgres --timeout=60s
kubectl -n vigilantai wait --for=condition=available deployment/redis --timeout=60s

# Apply application services
kubectl apply -f k8s/backend/
kubectl apply -f k8s/ai-service/
kubectl apply -f k8s/camera-gateway/
kubectl apply -f k8s/dashboard/

# Apply ingress
kubectl apply -f k8s/ingress.yaml

# Apply autoscalers
kubectl apply -f k8s/hpa/
```

### 3. Verify deployment

```bash
# Check all pods are running
kubectl -n vigilantai get pods

# Check all services
kubectl -n vigilantai get svc

# Check ingress
kubectl -n vigilantai get ingress

# Check HPAs
kubectl -n vigilantai get hpa

# Check logs
kubectl -n vigilantai logs -l app.kubernetes.io/name=backend -f
kubectl -n vigilantai logs -l app.kubernetes.io/name=dashboard -f
```

### 4. Quick apply all

```bash
kubectl apply -f k8s/ -R
```

## Service Endpoints

| Service | Internal URL | Health Check |
|---------|-------------|--------------|
| Backend | `http://backend:8080` | `/api/v1/health` |
| AI Service | `http://ai-service:8081` | `/health` |
| Camera Gateway | `http://camera-gateway:8082` | `/health` |
| Dashboard | `http://dashboard:3000` | `/` |
| PostgreSQL | `postgres:5432` | `pg_isready` |
| Redis | `redis:6379` | `redis-cli ping` |

## Resource Limits

| Service | CPU Request | CPU Limit | Memory Request | Memory Limit |
|---------|------------|-----------|----------------|--------------|
| Backend | 500m | 2 | 512Mi | 2Gi |
| AI Service | 500m | 2 | 1Gi | 4Gi |
| Camera Gateway | 250m | 1 | 256Mi | 1Gi |
| Dashboard | 100m | 500m | 128Mi | 512Mi |
| PostgreSQL | 250m | 1 | 256Mi | 1Gi |
| Redis | 100m | 500m | 128Mi | 512Mi |

## Persistent Storage

| PVC | Capacity | Mount Path |
|-----|----------|------------|
| vigilantai-postgres-pvc | 10Gi | `/var/lib/postgresql/data` |
| vigilantai-redis-pvc | 2Gi | `/data` |
| vigilantai-evidence-pvc | 50Gi | `/data/evidence` |

## Autoscaling

| Service | Min | Max | CPU Target |
|---------|-----|-----|------------|
| Backend | 2 | 10 | 70% |
| AI Service | 1 | 5 | 70% |
| Camera Gateway | 1 | 5 | 70% |

## Cleanup

```bash
kubectl delete namespace vigilantai
```

## Troubleshooting

### Pods stuck in Pending
```bash
kubectl -n vigilantai describe pod <pod-name>
kubectl -n vigilantai get events --sort-by='.lastTimestamp'
```

### Check configmap values
```bash
kubectl -n vigilantai get configmap vigilantai-config -o yaml
```

### Check secret values (base64)
```bash
kubectl -n vigilantai get secret vigilantai-secrets -o jsonpath='{.data.POSTGRES_PASSWORD}' | base64 -d
```

### Restart a deployment
```bash
kubectl -n vigilantai rollout restart deployment/backend
```

### Check rollout status
```bash
kubectl -n vigilantai rollout status deployment/backend
```

### Scale manually
```bash
kubectl -n vigilantai scale deployment/backend --replicas=3
```
