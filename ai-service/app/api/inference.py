"""Inference endpoints."""

from fastapi import APIRouter, Depends, HTTPException

from app.dependencies import get_inference_service
from app.logging import get_logger
from app.schemas.detection import (
    BatchDetectionRequest,
    BatchDetectionResponse,
    DetectionRequest,
    DetectionResponse,
)
from app.services.inference_service import InferenceService

router = APIRouter(prefix="/inference", tags=["inference"])
logger = get_logger(__name__)


@router.post("", response_model=DetectionResponse)
async def detect(
    request: DetectionRequest,
    inference_service: InferenceService = Depends(get_inference_service),
) -> DetectionResponse:
    """Run object detection on a single image.

    Args:
        request: Detection request with image URL and parameters.

    Returns:
        DetectionResponse with detection results.

    Raises:
        HTTPException: 501 Not Implemented (placeholder).
    """
    try:
        return await inference_service.detect(request)
    except NotImplementedError as e:
        raise HTTPException(status_code=501, detail=str(e))


@router.post("/batch", response_model=BatchDetectionResponse)
async def detect_batch(
    request: BatchDetectionRequest,
    inference_service: InferenceService = Depends(get_inference_service),
) -> BatchDetectionResponse:
    """Run object detection on multiple images.

    Args:
        request: Batch detection request with multiple image URLs.

    Returns:
        BatchDetectionResponse with detection results.

    Raises:
        HTTPException: 501 Not Implemented (placeholder).
    """
    try:
        return await inference_service.detect_batch(request)
    except NotImplementedError as e:
        raise HTTPException(status_code=501, detail=str(e))
