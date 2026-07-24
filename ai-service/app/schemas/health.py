"""Health-related schemas."""

from enum import Enum

from pydantic import BaseModel, Field


class ServiceStatus(str, Enum):
    """Service status enumeration."""

    HEALTHY = "healthy"
    UNHEALTHY = "unhealthy"
    DEGRADED = "degraded"


class ModelStatus(str, Enum):
    """Model status enumeration."""

    LOADED = "loaded"
    NOT_LOADED = "not_loaded"
    LOADING = "loading"
    ERROR = "error"


class ModelInfo(BaseModel):
    """Information about the loaded model."""

    name: str = Field(..., description="Model name")
    version: str = Field(..., description="Model version")
    status: ModelStatus = Field(..., description="Model status")
    device: str = Field(..., description="Device (cpu/cuda)")
    input_shape: list[int] | None = Field(None, description="Expected input shape")
    class_count: int | None = Field(None, description="Number of classes")


class HealthResponse(BaseModel):
    """Health check response."""

    status: ServiceStatus = Field(..., description="Service status")
    version: str = Field(..., description="Service version")
    uptime_seconds: float = Field(..., ge=0, description="Uptime in seconds")
    model: ModelInfo = Field(..., description="Model information")


class DetailedHealthResponse(BaseModel):
    """Detailed health check response."""

    status: ServiceStatus = Field(..., description="Service status")
    version: str = Field(..., description="Service version")
    uptime_seconds: float = Field(..., ge=0, description="Uptime in seconds")
    model: ModelInfo = Field(..., description="Model information")
    request_count: int = Field(..., ge=0, description="Total request count")
    successful_requests: int = Field(..., ge=0, description="Successful request count")
    failed_requests: int = Field(..., ge=0, description="Failed request count")
    average_inference_time_ms: float = Field(
        ..., ge=0, description="Average inference time in ms"
    )
    images_processed: int = Field(..., ge=0, description="Total images processed")
    total_detections: int = Field(..., ge=0, description="Total detections returned")
    average_detections_per_image: float = Field(
        ..., ge=0, description="Average detections per image"
    )
