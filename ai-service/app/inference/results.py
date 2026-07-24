"""Detection result schemas for YOLO inference output."""

from pydantic import BaseModel, ConfigDict, Field


class BoundingBox(BaseModel):
    """Bounding box with absolute and relative coordinates."""

    x1: float = Field(..., description="Left x coordinate (pixels)")
    y1: float = Field(..., description="Top y coordinate (pixels)")
    x2: float = Field(..., description="Right x coordinate (pixels)")
    y2: float = Field(..., description="Bottom y coordinate (pixels)")
    width: float = Field(..., ge=0, description="Box width (pixels)")
    height: float = Field(..., ge=0, description="Box height (pixels)")
    center_x: float = Field(..., description="Center x coordinate (pixels)")
    center_y: float = Field(..., description="Center y coordinate (pixels)")

    @classmethod
    def from_xyxy(cls, x1: float, y1: float, x2: float, y2: float) -> "BoundingBox":
        """Create BoundingBox from xyxy coordinates."""
        return cls(
            x1=x1,
            y1=y1,
            x2=x2,
            y2=y2,
            width=x2 - x1,
            height=y2 - y1,
            center_x=(x1 + x2) / 2.0,
            center_y=(y1 + y2) / 2.0,
        )


class Detection(BaseModel):
    """A single object detection result."""

    class_id: int = Field(..., ge=0, description="Class index")
    class_name: str = Field(..., description="Class label")
    confidence: float = Field(..., ge=0, le=1, description="Confidence score")
    bbox: BoundingBox = Field(..., description="Bounding box")


class ImageSize(BaseModel):
    """Original image dimensions."""

    width: int = Field(..., gt=0, description="Image width in pixels")
    height: int = Field(..., gt=0, description="Image height in pixels")


class InferenceMetadata(BaseModel):
    """Metadata about the inference request."""

    model_config = ConfigDict(protected_namespaces=())

    model_name: str = Field(..., description="Model used for inference")
    image_size: ImageSize = Field(..., description="Original image dimensions")
    source: str = Field(default="<unknown>", description="Image source identifier")
    confidence_threshold: float = Field(
        ..., ge=0, le=1, description="Applied confidence threshold"
    )
    iou_threshold: float = Field(..., ge=0, le=1, description="Applied IoU threshold")


class DetectionResponse(BaseModel):
    """Response containing detection results for a single image."""

    detections: list[Detection] = Field(
        default_factory=list, description="List of detections"
    )
    detection_count: int = Field(..., ge=0, description="Number of detections returned")
    image_size: ImageSize = Field(..., description="Original image dimensions")
    processing_time_ms: float = Field(
        ..., ge=0, description="Total processing time in milliseconds"
    )
    inference_time_ms: float = Field(
        ..., ge=0, description="Model inference time in milliseconds"
    )
    metadata: InferenceMetadata = Field(..., description="Inference metadata")
