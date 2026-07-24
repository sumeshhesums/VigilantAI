"""Model implementations for AI inference.

YOLOModel uses Ultralytics for real YOLO inference.
RTDETRModel and GroundingDINOModel are placeholder implementations.
"""

from __future__ import annotations

import asyncio
from typing import Any

import numpy as np

from app.logging import get_logger
from app.models.base import BaseModel

logger = get_logger(__name__)


class YOLOModel(BaseModel):
    """YOLO object detection model using Ultralytics.

    Supports YOLOv8/v9/v10 family models. Loads the Ultralytics
    YOLO model and exposes the underlying model for inference.
    """

    def __init__(
        self,
        name: str = "yolov8n",
        version: str = "8.0.0",
        device: str = "cpu",
        model_path: str = "",
    ) -> None:
        super().__init__(
            name=name,
            version=version,
            model_type="yolo",
            device=device,
            description=f"YOLO object detection model ({name})",
        )
        self._model_path = model_path
        self._yolo_model: Any = None

    @property
    def yolo_model(self) -> Any:
        """Access the underlying Ultralytics YOLO model.

        Returns None if model is not loaded.
        """
        return self._yolo_model

    def predict(
        self,
        source: np.ndarray | Any,
        conf: float = 0.5,
        iou: float = 0.45,
        verbose: bool = False,
    ) -> list:
        """Run prediction using the loaded YOLO model.

        Args:
            source: Input image (numpy array, path, etc.).
            conf: Confidence threshold.
            iou: IoU threshold for NMS.
            verbose: Whether to print verbose output.

        Returns:
            List of Ultralytics Results objects.

        Raises:
            RuntimeError: If model is not loaded.
        """
        if self._yolo_model is None:
            raise RuntimeError(f"Model {self.name} is not loaded")
        return self._yolo_model.predict(
            source=source, conf=conf, iou=iou, verbose=verbose
        )

    async def load_model(self) -> None:
        """Load the YOLO model using Ultralytics."""
        logger.info(
            "Loading YOLO model: %s (device=%s)",
            self.name,
            self._metadata.device,
        )

        def _load() -> Any:
            from ultralytics import YOLO

            path = self._model_path if self._model_path else f"{self.name}.pt"
            model = YOLO(path)
            return model

        loop = asyncio.get_event_loop()
        self._yolo_model = await loop.run_in_executor(None, _load)

        # Extract metadata from the loaded model
        if self._yolo_model is not None:
            self._metadata.input_shape = [1, 3, 640, 640]
            self._metadata.class_count = (
                len(self._yolo_model.names)
                if hasattr(self._yolo_model, "names")
                else 80
            )
            self._metadata.extra = {
                "framework": "ultralytics",
                "task": "detect",
                "names": (
                    dict(self._yolo_model.names)
                    if hasattr(self._yolo_model, "names")
                    else {}
                ),
            }

        logger.info(
            "YOLO model %s loaded (%d classes)",
            self.name,
            self._metadata.class_count or 0,
        )

    async def unload_model(self) -> None:
        """Unload the YOLO model and release resources."""
        logger.info("Unloading YOLO model: %s", self.name)
        self._yolo_model = None
        self._metadata.input_shape = None
        self._metadata.class_count = None
        self._metadata.extra = {}

    async def warmup(self):  # type: ignore[override]
        """Warm up the model with a dummy inference pass."""
        if not self.is_loaded:
            raise RuntimeError(f"Model {self.name} must be loaded first")

        from app.models.base import ModelState

        self._metadata.state = ModelState.WARMING_UP
        logger.info("Warming up YOLO model: %s", self.name)

        try:
            dummy = np.zeros((640, 640, 3), dtype=np.uint8)
            await asyncio.get_event_loop().run_in_executor(
                None,
                lambda: self._yolo_model.predict(source=dummy, verbose=False),
            )
        except Exception as e:
            logger.warning("Warmup prediction failed (non-fatal): %s", e)

        self._metadata.state = ModelState.LOADED
        logger.info("YOLO model %s warmup complete", self.name)
        return self._metadata


class RTDETRModel(BaseModel):
    """RT-DETR (Real-Time Detection Transformer) model.

    Placeholder implementation for RT-DETR family models.
    """

    def __init__(
        self,
        name: str = "rtdetr-l",
        version: str = "1.0.0",
        device: str = "cpu",
    ) -> None:
        super().__init__(
            name=name,
            version=version,
            model_type="rtdetr",
            device=device,
            description=f"RT-DETR transformer detection model ({name})",
        )

    async def load_model(self) -> None:
        """Simulate loading an RT-DETR model."""
        logger.info("Simulating RT-DETR model load: %s", self.name)
        await asyncio.sleep(0.05)
        self._metadata.input_shape = [1, 3, 640, 640]
        self._metadata.class_count = 80
        self._metadata.extra = {"framework": "ultralytics", "task": "detect"}

    async def unload_model(self) -> None:
        """Simulate unloading an RT-DETR model."""
        logger.info("Simulating RT-DETR model unload: %s", self.name)
        self._metadata.input_shape = None
        self._metadata.class_count = None
        self._metadata.extra = {}


class GroundingDINOModel(BaseModel):
    """Grounding DINO open-set object detection model.

    Placeholder implementation for Grounding DINO family models.
    """

    def __init__(
        self,
        name: str = "grounding-dino-tiny",
        version: str = "1.0.0",
        device: str = "cpu",
    ) -> None:
        super().__init__(
            name=name,
            version=version,
            model_type="grounding_dino",
            device=device,
            description=f"Grounding DINO open-set detection model ({name})",
        )

    async def load_model(self) -> None:
        """Simulate loading a Grounding DINO model."""
        logger.info("Simulating Grounding DINO model load: %s", self.name)
        await asyncio.sleep(0.05)
        self._metadata.input_shape = [1, 3, 800, 800]
        self._metadata.class_count = None
        self._metadata.extra = {
            "framework": "transformers",
            "task": "open-set-detect",
        }

    async def unload_model(self) -> None:
        """Simulate unloading a Grounding DINO model."""
        logger.info("Simulating Grounding DINO model unload: %s", self.name)
        self._metadata.input_shape = None
        self._metadata.class_count = None
        self._metadata.extra = {}
