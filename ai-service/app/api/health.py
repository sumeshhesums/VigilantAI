"""Health check endpoint."""

from fastapi import APIRouter, Depends

from app.config import get_settings
from app.core.metrics import MetricsManager
from app.core.model_manager import ModelManager
from app.dependencies import get_metrics_manager, get_model_manager
from app.logging import get_logger
from app.schemas.health import (
    DetailedHealthResponse,
    HealthResponse,
    ModelInfo,
    ModelStatus,
    ServiceStatus,
)

router = APIRouter(tags=["health"])
logger = get_logger(__name__)

_STATE_TO_STATUS = {
    "loaded": ModelStatus.LOADED,
    "not_loaded": ModelStatus.NOT_LOADED,
    "loading": ModelStatus.LOADING,
    "error": ModelStatus.ERROR,
}


def _build_model_info(model_manager: ModelManager) -> ModelInfo:
    """Build ModelInfo from the active model."""
    info = model_manager.get_model_info()
    status_raw = info.get("status", "not_loaded")
    return ModelInfo(
        name=info["name"],
        version=info["version"],
        status=_STATE_TO_STATUS.get(status_raw, ModelStatus.NOT_LOADED),
        device=info["device"],
        input_shape=info.get("input_shape"),
        class_count=info.get("class_count"),
    )


@router.get("/health", response_model=HealthResponse)
async def health_check(
    model_manager: ModelManager = Depends(get_model_manager),
    metrics_manager: MetricsManager = Depends(get_metrics_manager),
) -> HealthResponse:
    """Basic health check endpoint."""
    settings = get_settings()
    return HealthResponse(
        status=ServiceStatus.HEALTHY,
        version=settings.SERVICE_VERSION,
        uptime_seconds=metrics_manager.uptime_seconds,
        model=_build_model_info(model_manager),
    )


@router.get("/health/detailed", response_model=DetailedHealthResponse)
async def detailed_health_check(
    model_manager: ModelManager = Depends(get_model_manager),
    metrics_manager: MetricsManager = Depends(get_metrics_manager),
) -> DetailedHealthResponse:
    """Detailed health check with request metrics."""
    settings = get_settings()
    snapshot = metrics_manager.get_snapshot()
    return DetailedHealthResponse(
        status=ServiceStatus.HEALTHY,
        version=settings.SERVICE_VERSION,
        uptime_seconds=snapshot.uptime_seconds,
        model=_build_model_info(model_manager),
        request_count=snapshot.request_count,
        successful_requests=snapshot.successful_requests,
        failed_requests=snapshot.failed_requests,
        average_inference_time_ms=snapshot.average_inference_time_ms,
    )
