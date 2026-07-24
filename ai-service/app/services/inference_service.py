"""Inference service for object detection."""

import time

from app.core.model_manager import ModelManager
from app.core.metrics import MetricsManager
from app.logging import get_logger
from app.schemas.detection import (
    BatchDetectionRequest,
    BatchDetectionResponse,
    DetectionRequest,
    DetectionResponse,
)

logger = get_logger(__name__)


class InferenceService:
    """Service for running object detection inference."""

    def __init__(
        self,
        model_manager: ModelManager,
        metrics_manager: MetricsManager,
    ) -> None:
        self._model_manager = model_manager
        self._metrics_manager = metrics_manager

    async def health(self) -> dict:
        """Get inference service health status.

        Returns:
            Dictionary with health information.
        """
        return {
            "model_loaded": self._model_manager.is_loaded,
            "model_info": self._model_manager.get_model_info(),
        }

    async def detect(self, request: DetectionRequest) -> DetectionResponse:
        """Run object detection on a single image.

        Args:
            request: Detection request with image URL.

        Returns:
            DetectionResponse with detection results.

        Raises:
            NotImplementedError: Always raised as placeholder.
        """
        start_time = time.time()

        try:
            # Placeholder: In production, run actual inference
            # e.g., results = model.predict(image)
            raise NotImplementedError(
                "Object detection not yet implemented. "
                "Model inference will be available in a future version."
            )
        except NotImplementedError:
            raise
        except Exception as e:
            processing_time_ms = (time.time() - start_time) * 1000
            self._metrics_manager.record_request(
                success=False, inference_time_ms=processing_time_ms
            )
            logger.error("Detection failed: %s", e)
            raise

    async def detect_batch(
        self, request: BatchDetectionRequest
    ) -> BatchDetectionResponse:
        """Run object detection on multiple images.

        Args:
            request: Batch detection request with multiple image URLs.

        Returns:
            BatchDetectionResponse with detection results.

        Raises:
            NotImplementedError: Always raised as placeholder.
        """
        start_time = time.time()

        try:
            # Placeholder: In production, run batch inference
            raise NotImplementedError(
                "Batch object detection not yet implemented. "
                "Batch inference will be available in a future version."
            )
        except NotImplementedError:
            raise
        except Exception as e:
            processing_time_ms = (time.time() - start_time) * 1000
            self._metrics_manager.record_request(
                success=False, inference_time_ms=processing_time_ms
            )
            logger.error("Batch detection failed: %s", e)
            raise
