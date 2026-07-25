"""YOLO object detector using Ultralytics."""

from __future__ import annotations

import asyncio
import time

import numpy as np

from app.config import Settings, get_settings
from app.inference.parser import ResultParser
from app.inference.results import DetectionResponse
from app.logging import get_logger
from app.preprocessing import PreprocessingPipeline

logger = get_logger(__name__)


class YoloDetector:
    """Runs YOLO inference on images using Ultralytics.

    Accepts raw image bytes, uses the preprocessing pipeline for
    validation and metadata, and delegates to Ultralytics for inference.
    """

    def __init__(
        self,
        settings: Settings | None = None,
    ) -> None:
        """Initialize the detector.

        Args:
            settings: Application settings. Uses defaults if None.
        """
        self._settings = settings or get_settings()
        self._parser = ResultParser()
        self._pipeline = PreprocessingPipeline()

    @property
    def settings(self) -> Settings:
        """Access application settings."""
        return self._settings

    @property
    def parser(self) -> ResultParser:
        """Access the result parser."""
        return self._parser

    @property
    def pipeline(self) -> PreprocessingPipeline:
        """Access the preprocessing pipeline."""
        return self._pipeline

    async def detect(
        self,
        image_bytes: bytes,
        model_name: str,
        model,
        source: str = "<unknown>",
        confidence_threshold: float | None = None,
        iou_threshold: float | None = None,
    ) -> DetectionResponse:
        """Run detection on raw image bytes.

        Args:
            image_bytes: Raw image bytes (JPEG, PNG, BMP).
            model_name: Name of the model for metadata.
            model: Ultralytics YOLO model instance.
            source: Image source identifier.
            confidence_threshold: Override default confidence.
            iou_threshold: Override default IoU threshold.

        Returns:
            DetectionResponse with detection results.

        Raises:
            ValueError: If image is invalid or empty.
            RuntimeError: If inference fails.
        """
        conf = confidence_threshold or self._settings.CONFIDENCE_THRESHOLD
        iou = iou_threshold or self._settings.IOU_THRESHOLD

        start_time = time.perf_counter()

        if not image_bytes:
            raise ValueError("Image data is empty")

        # Validate image through preprocessing pipeline
        loaded = self._pipeline.loader.load_from_bytes(image_bytes, source=source)
        self._pipeline.validator.validate_or_raise(loaded)

        image_width = loaded.width
        image_height = loaded.height

        # Run YOLO inference via thread pool (Ultralytics is sync)
        inference_start = time.perf_counter()
        try:
            raw_results = await asyncio.wait_for(
                asyncio.get_event_loop().run_in_executor(
                    None,
                    lambda: model.predict(
                        source=np.array(loaded.data),
                        conf=conf,
                        iou=iou,
                        verbose=False,
                    ),
                ),
                timeout=self._settings.INFERENCE_TIMEOUT,
            )
        except asyncio.TimeoutError:
            raise RuntimeError(
                f"Inference timed out after {self._settings.INFERENCE_TIMEOUT}s"
            ) from None
        except Exception as e:
            raise RuntimeError(f"Inference failed: {e}") from e

        inference_time_ms = (time.perf_counter() - inference_start) * 1000
        total_time_ms = (time.perf_counter() - start_time) * 1000

        # Parse results
        response = self._parser.parse(
            results=raw_results,
            image_width=image_width,
            image_height=image_height,
            model_name=model_name,
            confidence_threshold=conf,
            iou_threshold=iou,
            source=source,
        )

        # Inject timing
        response.processing_time_ms = round(total_time_ms, 2)
        response.inference_time_ms = round(inference_time_ms, 2)

        logger.info(
            "Detection complete: %d objects in %.1fms (inference=%.1fms)",
            response.detection_count,
            total_time_ms,
            inference_time_ms,
        )

        return response

    async def detect_batch(
        self,
        images: list[tuple[int, bytes, str]],
        model_name: str,
        model,
        confidence_threshold: float | None = None,
        iou_threshold: float | None = None,
    ) -> list[tuple[int, str, DetectionResponse | None, str | None]]:
        """Run detection on multiple images concurrently.

        Args:
            images: List of (index, image_bytes, source) tuples.
            model_name: Name of the model for metadata.
            model: Ultralytics YOLO model instance.
            confidence_threshold: Override default confidence.
            iou_threshold: Override default IoU threshold.

        Returns:
            List of (index, source, response_or_none, error_or_none) tuples.
        """

        async def _process_one(
            idx: int, img_bytes: bytes, src: str
        ) -> tuple[int, str, DetectionResponse | None, str | None]:
            try:
                resp = await self.detect(
                    image_bytes=img_bytes,
                    model_name=model_name,
                    model=model,
                    source=src,
                    confidence_threshold=confidence_threshold,
                    iou_threshold=iou_threshold,
                )
                return (idx, src, resp, None)
            except Exception as e:
                return (idx, src, None, str(e))

        tasks = [_process_one(idx, data, src) for idx, data, src in images]
        return await asyncio.gather(*tasks)
