"""Prometheus metrics endpoint."""
from fastapi import APIRouter, Response
from prometheus_client import (
    CollectorRegistry,
    Counter,
    Gauge,
    Histogram,
    generate_latest,
    multiprocess,
)
from prometheus_client import CONTENT_TYPE_LATEST
import os

router = APIRouter(tags=["metrics"])

PROMETHEUS_MULTIPROC_DIR = os.environ.get("PROMETHEUS_MULTIPROC_DIR", "")

registry = CollectorRegistry()

if PROMETHEUS_MULTIPROC_DIR:
    registry = CollectorRegistry()
    multiprocess.MultiProcessCollector(registry)

# -- AI Service Prometheus Metrics --
INFERENCE_REQUESTS_TOTAL = Counter(
    "vigilantai_ai_inference_requests_total",
    "Total inference requests",
    registry=registry,
)
INFERENCE_FAILURES_TOTAL = Counter(
    "vigilantai_ai_inference_failures_total",
    "Total inference failures",
    registry=registry,
)
INFERENCE_LATENCY_SECONDS = Histogram(
    "vigilantai_ai_inference_latency_seconds",
    "Inference latency in seconds",
    buckets=[0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0],
    registry=registry,
)
IMAGES_PROCESSED_TOTAL = Counter(
    "vigilantai_ai_images_processed_total",
    "Total images processed",
    registry=registry,
)
DETECTIONS_TOTAL = Counter(
    "vigilantai_ai_detections_total",
    "Total detections produced",
    registry=registry,
)
MODEL_LOAD_TIME_SECONDS = Histogram(
    "vigilantai_ai_model_load_time_seconds",
    "Model load time in seconds",
    buckets=[0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0, 30.0, 60.0],
    registry=registry,
)
DETECTION_CONFIDENCE = Histogram(
    "vigilantai_ai_detection_confidence",
    "Detection confidence distribution",
    buckets=[0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9, 0.95, 0.99],
    registry=registry,
)
CPU_USAGE = Gauge(
    "vigilantai_ai_cpu_usage_percent",
    "CPU usage percentage",
    registry=registry,
)
MEMORY_USAGE = Gauge(
    "vigilantai_ai_memory_usage_bytes",
    "Memory usage in bytes",
    registry=registry,
)


@router.get("/metrics")
async def prometheus_metrics() -> Response:
    """Expose Prometheus metrics."""
    if PROMETHEUS_MULTIPROC_DIR:
        return Response(
            content=generate_latest(registry),
            media_type=CONTENT_TYPE_LATEST,
        )
    return Response(
        content=generate_latest(),
        media_type=CONTENT_TYPE_LATEST,
    )
