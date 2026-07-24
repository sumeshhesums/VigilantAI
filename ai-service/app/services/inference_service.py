"""Inference service for object detection."""

import time

from app.core.model_manager import ModelManager
from app.core.metrics import MetricsManager
from app.inference.detector import YoloDetector
from app.inference.results import DetectionResponse
from app.logging import get_logger
from app.models.factory import YOLOModel

logger = get_logger(__name__)


class InferenceService:
    """Service for running object detection inference.

    Coordinates the YoloDetector with the ModelManager to process
    images through the active YOLO model.
    """

    def __init__(
        self,
        model_manager: ModelManager,
        metrics_manager: MetricsManager,
        detector: YoloDetector | None = None,
    ) -> None:
        self._model_manager = model_manager
        self._metrics_manager = metrics_manager
        self._detector = detector or YoloDetector()

    @property
    def detector(self) -> YoloDetector:
        """Access the YOLO detector."""
        return self._detector

    async def health(self) -> dict:
        """Get inference service health status."""
        return {
            "model_loaded": self._model_manager.is_loaded,
            "model_info": self._model_manager.get_model_info(),
        }

    async def detect(
        self,
        image_bytes: bytes,
        source: str = "<unknown>",
        confidence_threshold: float | None = None,
        iou_threshold: float | None = None,
    ) -> DetectionResponse:
        """Run object detection on a single image.

        Args:
            image_bytes: Raw image bytes (JPEG, PNG, BMP).
            source: Image source identifier.
            confidence_threshold: Override default confidence threshold.
            iou_threshold: Override default IoU threshold.

        Returns:
            DetectionResponse with detection results.

        Raises:
            RuntimeError: If no model loaded or inference fails.
            ValueError: If image is invalid.
        """
        model = self._model_manager.active_model

        if model is None or not model.is_loaded:
            raise RuntimeError("No model loaded for inference (model not ready)")

        if not isinstance(model, YOLOModel):
            raise RuntimeError(f"Active model '{model.name}' is not a YOLO model")

        start_time = time.perf_counter()

        try:
            response = await self._detector.detect(
                image_bytes=image_bytes,
                model_name=model.name,
                model=model,
                source=source,
                confidence_threshold=confidence_threshold,
                iou_threshold=iou_threshold,
            )

            elapsed_ms = (time.perf_counter() - start_time) * 1000
            response.processing_time_ms = round(elapsed_ms, 2)

            self._metrics_manager.record_inference(
                success=True,
                inference_time_ms=elapsed_ms,
                detection_count=response.detection_count,
            )

            return response

        except Exception as e:
            elapsed_ms = (time.perf_counter() - start_time) * 1000
            self._metrics_manager.record_inference(
                success=False,
                inference_time_ms=elapsed_ms,
                detection_count=0,
            )
            logger.error("Detection failed: %s", e)
            raise
