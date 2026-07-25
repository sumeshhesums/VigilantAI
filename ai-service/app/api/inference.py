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

MAX_BATCH_SIZE = 32


@router.post("", response_model=dict)
async def detect(
    file: UploadFile = File(..., description="Image file to analyze"),
    confidence_threshold: float | None = Form(
        None, description="Confidence threshold (0.0-1.0)"
    ),
    iou_threshold: float | None = Form(None, description="IoU threshold (0.0-1.0)"),
    text_prompt: str | None = Form(
        None,
        description="Text prompt for open-set detection (GroundingDINO only). "
        "Separate classes with dots, e.g. 'person . car . dog'",
    ),
    inference_service: InferenceService = Depends(get_inference_service),
) -> dict:
    """Run object detection on a single image.

    Accepts a multipart/form-data upload with an image file.
    For GroundingDINO, provide a text_prompt to specify what to detect.

    Args:
        file: Image file (JPEG, PNG, BMP).
        confidence_threshold: Override default confidence threshold.
        iou_threshold: Override default IoU threshold.
        text_prompt: Optional text prompt for open-set detection (GroundingDINO).

    Returns:
        DetectionResponse with detection results.

    Raises:
        HTTPException: 400 for invalid input, 500 for server errors.
    """
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

    kwargs = {}
    if text_prompt:
        kwargs["text_prompt"] = text_prompt

    try:
        response = await inference_service.detect(
            image_bytes=image_bytes,
            source=file.filename or "<upload>",
            confidence_threshold=confidence_threshold,
            iou_threshold=iou_threshold,
            **kwargs,
        )
        return response.model_dump()
    except ValueError as e:
        raise HTTPException(status_code=400, detail=f"Invalid input: {e}")
    except RuntimeError as e:
        error_msg = str(e)
        error_lower = error_msg.lower()
        if "not loaded" in error_lower or "no model loaded" in error_lower:
            raise HTTPException(status_code=503, detail=error_msg)
        if "timed out" in error_lower:
            raise HTTPException(status_code=504, detail=error_msg)
        raise HTTPException(status_code=500, detail=error_msg)
    except Exception as e:
        logger.error("Inference failed: %s", e)
        raise HTTPException(status_code=500, detail=f"Inference failed: {e}")


@router.post("/batch", response_model=dict)
async def detect_batch(
    files: list[UploadFile] = File(..., description="Image files to analyze (max 32)"),
    confidence_threshold: float | None = Form(
        None, description="Confidence threshold (0.0-1.0)"
    ),
    iou_threshold: float | None = Form(None, description="IoU threshold (0.0-1.0)"),
    text_prompt: str | None = Form(
        None,
        description="Text prompt for open-set detection (GroundingDINO only). "
        "Separate classes with dots, e.g. 'person . car . dog'",
    ),
    inference_service: InferenceService = Depends(get_inference_service),
) -> dict:
    """Run object detection on multiple images.

    Accepts a multipart/form-data upload with multiple image files.
    Images are processed concurrently for throughput.
    For GroundingDINO, provide a text_prompt to specify what to detect.

    Args:
        files: List of image files (JPEG, PNG, BMP). Max 32.
        confidence_threshold: Override default confidence threshold.
        iou_threshold: Override default IoU threshold.
        text_prompt: Optional text prompt for open-set detection (GroundingDINO).

    Returns:
        BatchDetectionResponse with results for each image.

    Raises:
        HTTPException: 400 for invalid input, 503 if model not ready, 500 for server errors.
    """
    if not files:
        raise HTTPException(status_code=400, detail="No files provided")

    if len(files) > MAX_BATCH_SIZE:
        raise HTTPException(
            status_code=400,
            detail=f"Batch size {len(files)} exceeds maximum of {MAX_BATCH_SIZE}",
        )

    images: list[tuple[int, bytes, str]] = []
    for idx, file in enumerate(files):
        if file.content_type and file.content_type not in SUPPORTED_CONTENT_TYPES:
            raise HTTPException(
                status_code=400,
                detail=f"File {idx} ({file.filename}): unsupported content type "
                f"{file.content_type}. Supported: {sorted(SUPPORTED_CONTENT_TYPES)}",
            )
        try:
            data = await file.read()
        except Exception as e:
            raise HTTPException(
                status_code=400,
                detail=f"File {idx} ({file.filename}): failed to read: {e}",
            )
        if not data:
            raise HTTPException(
                status_code=400,
                detail=f"File {idx} ({file.filename}): file is empty",
            )
        images.append((idx, data, file.filename or f"upload_{idx}"))

    kwargs = {}
    if text_prompt:
        kwargs["text_prompt"] = text_prompt

    try:
        response = await inference_service.detect_batch(
            images=images,
            confidence_threshold=confidence_threshold,
            iou_threshold=iou_threshold,
            **kwargs,
        )
        return response.model_dump()
    except RuntimeError as e:
        error_msg = str(e)
        error_lower = error_msg.lower()
        if "not loaded" in error_lower or "no model loaded" in error_lower:
            raise HTTPException(status_code=503, detail=error_msg)
        if "timed out" in error_lower:
            raise HTTPException(status_code=504, detail=error_msg)
        raise HTTPException(status_code=500, detail=error_msg)
    except Exception as e:
        logger.error("Batch inference failed: %s", e)
        raise HTTPException(status_code=500, detail=f"Batch inference failed: {e}")
