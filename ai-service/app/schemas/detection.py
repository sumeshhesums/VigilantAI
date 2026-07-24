"""Detection-related schemas."""

from pydantic import BaseModel, Field


class BoundingBox(BaseModel):
    """Bounding box coordinates."""

    x_min: float = Field(..., ge=0, description="Left x coordinate")
    y_min: float = Field(..., ge=0, description="Top y coordinate")
    x_max: float = Field(..., ge=0, description="Right x coordinate")
    y_max: float = Field(..., ge=0, description="Bottom y coordinate")


class Detection(BaseModel):
    """A single detection result."""

    class_name: str = Field(..., description="Detected class name")
    confidence: float = Field(..., ge=0, le=1, description="Confidence score")
    bbox: BoundingBox = Field(..., description="Bounding box")


class DetectionRequest(BaseModel):
    """Request for object detection."""

    image_url: str = Field(..., description="URL of the image to analyze")
    camera_id: str | None = Field(None, description="Camera identifier")
    confidence_threshold: float = Field(
        0.5, ge=0, le=1, description="Minimum confidence threshold"
    )


class DetectionResponse(BaseModel):
    """Response containing detection results."""

    detections: list[Detection] = Field(
        default_factory=list, description="List of detections"
    )
    image_width: int = Field(..., gt=0, description="Image width in pixels")
    image_height: int = Field(..., gt=0, description="Image height in pixels")
    processing_time_ms: float = Field(
        ..., ge=0, description="Processing time in milliseconds"
    )


class BatchDetectionRequest(BaseModel):
    """Batch request for object detection."""

    requests: list[DetectionRequest] = Field(
        ..., min_length=1, description="List of detection requests"
    )


class BatchDetectionResponse(BaseModel):
    """Batch response containing detection results."""

    results: list[DetectionResponse] = Field(
        default_factory=list, description="List of detection results"
    )
    total_processing_time_ms: float = Field(
        ..., ge=0, description="Total processing time in milliseconds"
    )
    successful_count: int = Field(
        ..., ge=0, description="Number of successful detections"
    )
    failed_count: int = Field(..., ge=0, description="Number of failed detections")
