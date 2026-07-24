"""Inference endpoints."""

from fastapi import APIRouter, Depends, File, Form, HTTPException, UploadFile

from app.dependencies import get_inference_service
from app.logging import get_logger
from app.services.inference_service import InferenceService

router = APIRouter(prefix="/inference", tags=["inference"])
logger = get_logger(__name__)

SUPPORTED_CONTENT_TYPES = {
    "image/jpeg",
    "image/jpg",
    "image/png",
    "image/bmp",
}


@router.post("", response_model=dict)
async def detect(
    file: UploadFile = File(..., description="Image file to analyze"),
    confidence_threshold: float | None = Form(
        None, description="Confidence threshold (0.0-1.0)"
    ),
    iou_threshold: float | None = Form(None, description="IoU threshold (0.0-1.0)"),
    inference_service: InferenceService = Depends(get_inference_service),
) -> dict:
    """Run object detection on a single image.

    Accepts a multipart/form-data upload with an image file.

    Args:
        file: Image file (JPEG, PNG, BMP).
        confidence_threshold: Override default confidence threshold.
        iou_threshold: Override default IoU threshold.

    Returns:
        DetectionResponse with detection results.

    Raises:
        HTTPException: 400 for invalid input, 500 for server errors.
    """
    # Validate content type
    if file.content_type and file.content_type not in SUPPORTED_CONTENT_TYPES:
        raise HTTPException(
            status_code=400,
            detail=f"Unsupported content type: {file.content_type}. "
            f"Supported: {sorted(SUPPORTED_CONTENT_TYPES)}",
        )

    try:
        image_bytes = await file.read()
    except Exception as e:
        raise HTTPException(
            status_code=400, detail=f"Failed to read uploaded file: {e}"
        )

    if not image_bytes:
        raise HTTPException(status_code=400, detail="Uploaded file is empty")

    try:
        response = await inference_service.detect(
            image_bytes=image_bytes,
            source=file.filename or "<upload>",
            confidence_threshold=confidence_threshold,
            iou_threshold=iou_threshold,
        )
        return response.model_dump()
    except ValueError as e:
        raise HTTPException(status_code=400, detail=f"Invalid image: {e}")
    except RuntimeError as e:
        error_msg = str(e)
        if "not loaded" in error_msg.lower():
            raise HTTPException(status_code=503, detail=error_msg)
        if "timed out" in error_msg.lower():
            raise HTTPException(status_code=504, detail=error_msg)
        raise HTTPException(status_code=500, detail=error_msg)
    except Exception as e:
        logger.error("Inference failed: %s", e)
        raise HTTPException(status_code=500, detail=f"Inference failed: {e}")


@router.post("/batch", response_model=dict)
async def detect_batch(
    inference_service: InferenceService = Depends(get_inference_service),
) -> dict:
    """Run object detection on multiple images (placeholder).

    Returns:
        501 Not Implemented.
    """
    raise HTTPException(status_code=501, detail="Batch inference not yet implemented")
