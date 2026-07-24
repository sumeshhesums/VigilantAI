"""Health check endpoint."""

from fastapi import APIRouter, Depends, Request

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


def _get_model_status(model_manager: ModelManager) -> ModelStatus:
    """Map model state to ModelStatus enum."""
    from app.core.model_manager import ModelState

    state_map = {
        ModelState.LOADED: ModelStatus.LOADED,
        ModelState.NOT_LOADED: ModelStatus.NOT_LOADED,
        ModelState.LOADING: ModelStatus.LOADING,
        ModelState.ERROR: ModelStatus.ERROR,
    }
    return state_map.get(model_manager.metadata.state, ModelStatus.NOT_LOADED)


@router.get("/health", response_model=HealthResponse)
async def health_check(
    request: Request,
    model_manager: ModelManager = Depends(get_model_manager),
    metrics_manager: MetricsManager = Depends(get_metrics_manager),
) -> HealthResponse:
    """Basic health check endpoint.

    Returns service status, version, uptime, and model info.
    """
    settings = get_settings()
    model_info = model_manager.get_model_info()

    return HealthResponse(
        status=ServiceStatus.HEALTHY,
        version=settings.SERVICE_VERSION,
        uptime_seconds=metrics_manager.uptime_seconds,
        model=ModelInfo(
            name=model_info["name"],
            version=model_info["version"],
            status=_get_model_status(model_manager),
            device=model_info["device"],
            input_shape=model_info.get("input_shape"),
            class_count=model_info.get("class_count"),
        ),
    )


@router.get("/health/detailed", response_model=DetailedHealthResponse)
async def detailed_health_check(
    request: Request,
    model_manager: ModelManager = Depends(get_model_manager),
    metrics_manager: MetricsManager = Depends(get_metrics_manager),
) -> DetailedHealthResponse:
    """Detailed health check with request metrics.

    Returns service status, version, uptime, model info, and request metrics.
    """
    settings = get_settings()
    snapshot = metrics_manager.get_snapshot()
    model_info = model_manager.get_model_info()

    return DetailedHealthResponse(
        status=ServiceStatus.HEALTHY,
        version=settings.SERVICE_VERSION,
        uptime_seconds=snapshot.uptime_seconds,
        model=ModelInfo(
            name=model_info["name"],
            version=model_info["version"],
            status=_get_model_status(model_manager),
            device=model_info["device"],
            input_shape=model_info.get("input_shape"),
            class_count=model_info.get("class_count"),
        ),
        request_count=snapshot.request_count,
        successful_requests=snapshot.successful_requests,
        failed_requests=snapshot.failed_requests,
        average_inference_time_ms=snapshot.average_inference_time_ms,
    )
