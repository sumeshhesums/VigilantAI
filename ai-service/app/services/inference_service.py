"""Inference service for object detection."""

from __future__ import annotations

import time
from typing import Any

from app.core.metrics import MetricsManager
from app.core.model_manager import ModelManager
from app.inference.results import BatchDetectionResponse, SingleBatchResult
from app.logging import get_logger

logger = get_logger(__name__)


class InferenceService:
    """Service for running object detection inference.

    Model-agnostic: retrieves the active model and calls predict()
    directly without checking concrete types.
    """

    def __init__(
        self,
        model_manager: ModelManager,
        metrics_manager: MetricsManager,
    ) -> None:
        self._model_manager = model_manager
        self._metrics_manager = metrics_manager

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
        **kwargs: Any,
    ) -> Any:
        """Run object detection on a single image.

        Args:
            image_bytes: Raw image bytes (JPEG, PNG, BMP).
            source: Image source identifier.
            confidence_threshold: Override default confidence threshold.
            iou_threshold: Override default IoU threshold.
            **kwargs: Model-specific parameters (e.g., text_prompt for GroundingDINO).

        Returns:
            DetectionResponse with detection results.

        Raises:
            RuntimeError: If no model loaded or inference fails.
            ValueError: If image is invalid.
        """
        model = self._model_manager.active_model

        if model is None or not model.is_loaded:
            raise RuntimeError("No model loaded for inference (model not ready)")

        start_time = time.perf_counter()

        try:
            response = await model.predict(
                image_bytes=image_bytes,
                confidence_threshold=confidence_threshold or 0.5,
                iou_threshold=iou_threshold or 0.45,
                source=source,
                **kwargs,
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

    async def detect_batch(
        self,
        images: list[tuple[int, bytes, str]],
        confidence_threshold: float | None = None,
        iou_threshold: float | None = None,
        **kwargs: Any,
    ) -> BatchDetectionResponse:
        """Run object detection on a batch of images.

        Args:
            images: List of (index, image_bytes, source) tuples.
            confidence_threshold: Override default confidence threshold.
            iou_threshold: Override default IoU threshold.
            **kwargs: Model-specific parameters (e.g., text_prompt for GroundingDINO).

        Returns:
            BatchDetectionResponse with results for each image.

        Raises:
            RuntimeError: If no model loaded.
        """
        import asyncio

        model = self._model_manager.active_model

        if model is None or not model.is_loaded:
            raise RuntimeError("No model loaded for inference (model not ready)")

        start_time = time.perf_counter()

        async def _process_one(
            idx: int, img_bytes: bytes, src: str
        ) -> tuple[int, str, Any | None, str | None]:
            try:
                resp = await model.predict(
                    image_bytes=img_bytes,
                    confidence_threshold=confidence_threshold,
                    iou_threshold=iou_threshold,
                    source=src,
                    **kwargs,
                )
                return (idx, src, resp, None)
            except Exception as e:
                return (idx, src, None, str(e))

        tasks = [_process_one(idx, data, src) for idx, data, src in images]
        raw_results = await asyncio.gather(*tasks)

        total_time_ms = (time.perf_counter() - start_time) * 1000

        results: list[SingleBatchResult] = []
        total_detections = 0
        successful = 0
        failed = 0

        for idx, source, detection, error in raw_results:
            if detection is not None:
                results.append(
                    SingleBatchResult(
                        index=idx, source=source, result=detection, error=None
                    )
                )
                total_detections += detection.detection_count
                successful += 1
                self._metrics_manager.record_inference(
                    success=True,
                    inference_time_ms=detection.inference_time_ms,
                    detection_count=detection.detection_count,
                )
            else:
                results.append(
                    SingleBatchResult(
                        index=idx, source=source, result=None, error=error
                    )
                )
                failed += 1
                self._metrics_manager.record_inference(
                    success=False,
                    inference_time_ms=0.0,
                    detection_count=0,
                )

        logger.info(
            "Batch inference complete: %d/%d images, %d total detections in %.1fms",
            successful,
            len(images),
            total_detections,
            total_time_ms,
        )

        return BatchDetectionResponse(
            results=results,
            total_images=len(images),
            successful=successful,
            failed=failed,
            total_detections=total_detections,
            total_processing_time_ms=round(total_time_ms, 2),
        )
