"""Placeholder model implementations.

These are skeleton implementations for future AI models.
They implement BaseModel with simulated loading only.
No inference, preprocessing, or GPU execution.
"""

import asyncio

from app.logging import get_logger
from app.models.base import BaseModel

logger = get_logger(__name__)


class YOLOModel(BaseModel):
    """YOLO (You Only Look Once) object detection model.

    Placeholder implementation for YOLOv8/v9/v10 family models.
    """

    def __init__(
        self,
        name: str = "yolov8n",
        version: str = "8.0.0",
        device: str = "cpu",
    ) -> None:
        super().__init__(
            name=name,
            version=version,
            model_type="yolo",
            device=device,
            description=f"YOLO object detection model ({name})",
        )

    async def load_model(self) -> None:
        """Simulate loading a YOLO model."""
        logger.info("Simulating YOLO model load: %s", self.name)
        await asyncio.sleep(0.05)
        self._metadata.input_shape = [1, 3, 640, 640]
        self._metadata.class_count = 80
        self._metadata.extra = {"framework": "ultralytics", "task": "detect"}

    async def unload_model(self) -> None:
        """Simulate unloading a YOLO model."""
        logger.info("Simulating YOLO model unload: %s", self.name)
        self._metadata.input_shape = None
        self._metadata.class_count = None
        self._metadata.extra = {}


class RTDETRModel(BaseModel):
    """RT-DETR (Real-Time Detection Transformer) model.

    placeholder implementation for RT-DETR family models.
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
        self._metadata.class_count = None  # Open-set: dynamic classes
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
