"""Model implementations for AI inference.

YOLOModel uses Ultralytics for YOLO inference.
RTDETRModel uses Ultralytics for RT-DETR inference.
GroundingDINOModel uses HuggingFace Transformers for open-set detection.
"""

from __future__ import annotations

import asyncio
from typing import Any

import cv2
import numpy as np

from app.logging import get_logger
from app.models.base import BaseModel

logger = get_logger(__name__)


def _load_image_from_bytes(image_bytes: bytes) -> np.ndarray:
    """Decode image bytes to a BGR numpy array.

    Args:
        image_bytes: Raw encoded image bytes.

    Returns:
        Decoded image as HWC BGR numpy array.

    Raises:
        ValueError: If image data is empty or cannot be decoded.
    """
    if not image_bytes:
        raise ValueError("Image data is empty")

    arr = np.frombuffer(image_bytes, dtype=np.uint8)
    img = cv2.imdecode(arr, cv2.IMREAD_COLOR)
    if img is None:
        raise ValueError("Failed to decode image from bytes")
    return img


def _get_image_dimensions(image_bytes: bytes) -> tuple[int, int]:
    """Get image width and height from bytes without full decode.

    Returns:
        (width, height) tuple.
    """
    img = _load_image_from_bytes(image_bytes)
    h, w = img.shape[:2]
    return w, h


class YOLOModel(BaseModel):
    """YOLO object detection model using Ultralytics.

    Supports YOLOv8/v9/v10 family models. Loads the Ultralytics
    YOLO model and exposes predict() returning DetectionResponse.
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
        """Access the underlying Ultralytics YOLO model."""
        return self._yolo_model

    async def predict(
        self,
        image_bytes: bytes,
        confidence_threshold: float = 0.5,
        iou_threshold: float = 0.45,
        source: str = "<unknown>",
        **kwargs: Any,
    ) -> Any:
        """Run YOLO inference on raw image bytes.

        Returns:
            DetectionResponse with detection results.
        """
        if self._yolo_model is None:
            raise RuntimeError(f"Model {self.name} is not loaded")

        from app.inference.parser import ResultParser

        img = _load_image_from_bytes(image_bytes)
        image_height, image_width = img.shape[:2]

        loop = asyncio.get_event_loop()
        raw_results = await asyncio.wait_for(
            loop.run_in_executor(
                None,
                lambda: self._yolo_model.predict(
                    source=img,
                    conf=confidence_threshold,
                    iou=iou_threshold,
                    verbose=False,
                ),
            ),
            timeout=kwargs.get("timeout", 10.0),
        )

        parser = ResultParser()
        return parser.parse(
            results=raw_results,
            image_width=image_width,
            image_height=image_height,
            model_name=self.name,
            confidence_threshold=confidence_threshold,
            iou_threshold=iou_threshold,
            source=source,
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
            return YOLO(path)

        loop = asyncio.get_event_loop()
        self._yolo_model = await loop.run_in_executor(None, _load)

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

    Uses Ultralytics RT-DETR for real-time transformer-based detection.
    Shares the same result format as YOLO via Ultralytics.
    """

    def __init__(
        self,
        name: str = "rtdetr-l",
        version: str = "1.0.0",
        device: str = "cpu",
        model_path: str = "",
    ) -> None:
        super().__init__(
            name=name,
            version=version,
            model_type="rtdetr",
            device=device,
            description=f"RT-DETR transformer detection model ({name})",
        )
        self._model_path = model_path
        self._rtdetr_model: Any = None

    @property
    def rtdetr_model(self) -> Any:
        """Access the underlying Ultralytics RTDETR model."""
        return self._rtdetr_model

    async def predict(
        self,
        image_bytes: bytes,
        confidence_threshold: float = 0.5,
        iou_threshold: float = 0.45,
        source: str = "<unknown>",
        **kwargs: Any,
    ) -> Any:
        """Run RT-DETR inference on raw image bytes.

        Returns:
            DetectionResponse with detection results.
        """
        if self._rtdetr_model is None:
            raise RuntimeError(f"Model {self.name} is not loaded")

        from app.inference.parser import ResultParser

        img = _load_image_from_bytes(image_bytes)
        image_height, image_width = img.shape[:2]

        loop = asyncio.get_event_loop()
        raw_results = await asyncio.wait_for(
            loop.run_in_executor(
                None,
                lambda: self._rtdetr_model.predict(
                    source=img,
                    conf=confidence_threshold,
                    iou=iou_threshold,
                    verbose=False,
                ),
            ),
            timeout=kwargs.get("timeout", 10.0),
        )

        parser = ResultParser()
        return parser.parse(
            results=raw_results,
            image_width=image_width,
            image_height=image_height,
            model_name=self.name,
            confidence_threshold=confidence_threshold,
            iou_threshold=iou_threshold,
            source=source,
        )

    async def load_model(self) -> None:
        """Load the RT-DETR model using Ultralytics."""
        logger.info(
            "Loading RT-DETR model: %s (device=%s)",
            self.name,
            self._metadata.device,
        )

        def _load() -> Any:
            from ultralytics import RTDETR

            path = self._model_path if self._model_path else f"{self.name}.pt"
            return RTDETR(path)

        loop = asyncio.get_event_loop()
        self._rtdetr_model = await loop.run_in_executor(None, _load)

        if self._rtdetr_model is not None:
            self._metadata.input_shape = [1, 3, 640, 640]
            self._metadata.class_count = (
                len(self._rtdetr_model.names)
                if hasattr(self._rtdetr_model, "names")
                else 80
            )
            self._metadata.extra = {
                "framework": "ultralytics",
                "task": "detect",
                "architecture": "rtdetr",
                "names": (
                    dict(self._rtdetr_model.names)
                    if hasattr(self._rtdetr_model, "names")
                    else {}
                ),
            }

        logger.info(
            "RT-DETR model %s loaded (%d classes)",
            self.name,
            self._metadata.class_count or 0,
        )

    async def unload_model(self) -> None:
        """Unload the RT-DETR model and release resources."""
        logger.info("Unloading RT-DETR model: %s", self.name)
        self._rtdetr_model = None
        self._metadata.input_shape = None
        self._metadata.class_count = None
        self._metadata.extra = {}

    async def warmup(self):  # type: ignore[override]
        """Warm up the model with a dummy inference pass."""
        if not self.is_loaded:
            raise RuntimeError(f"Model {self.name} must be loaded first")

        from app.models.base import ModelState

        self._metadata.state = ModelState.WARMING_UP
        logger.info("Warming up RT-DETR model: %s", self.name)

        try:
            dummy = np.zeros((640, 640, 3), dtype=np.uint8)
            await asyncio.get_event_loop().run_in_executor(
                None,
                lambda: self._rtdetr_model.predict(source=dummy, verbose=False),
            )
        except Exception as e:
            logger.warning("Warmup prediction failed (non-fatal): %s", e)

        self._metadata.state = ModelState.LOADED
        logger.info("RT-DETR model %s warmup complete", self.name)
        return self._metadata


class GroundingDINOModel(BaseModel):
    """Grounding DINO open-set object detection model.

    Uses HuggingFace Transformers to load the Grounding DINO model.
    Supports text-prompted open-set object detection.
    """

    def __init__(
        self,
        name: str = "grounding-dino-tiny",
        version: str = "1.0.0",
        device: str = "cpu",
        model_path: str = "",
    ) -> None:
        super().__init__(
            name=name,
            version=version,
            model_type="grounding_dino",
            device=device,
            description=f"Grounding DINO open-set detection model ({name})",
        )
        self._model_path = model_path
        self._processor: Any = None
        self._model: Any = None

    def _resolve_model_id(self) -> str:
        """Resolve HuggingFace model ID from config name."""
        mapping = {
            "grounding-dino-tiny": "IDEA-Research/grounding-dino-tiny",
            "grounding-dino-base": "IDEA-Research/grounding-dino-base",
        }
        if self._model_path:
            return self._model_path
        return mapping.get(self.name, "IDEA-Research/grounding-dino-tiny")

    async def predict(
        self,
        image_bytes: bytes,
        confidence_threshold: float = 0.5,
        iou_threshold: float = 0.45,
        source: str = "<unknown>",
        **kwargs: Any,
    ) -> Any:
        """Run Grounding DINO inference on raw image bytes.

        Args:
            image_bytes: Raw image bytes.
            confidence_threshold: Minimum detection confidence.
            iou_threshold: IoU threshold (unused for GroundingDINO postprocessing).
            source: Image source identifier.
            **kwargs: Must include 'text_prompt' (str) with detection targets.

        Returns:
            DetectionResponse with detection results.

        Raises:
            RuntimeError: If model is not loaded.
            ValueError: If no text_prompt provided or image is invalid.
        """
        if self._model is None or self._processor is None:
            raise RuntimeError(f"Model {self.name} is not loaded")

        text_prompt = kwargs.get("text_prompt", "")
        if not text_prompt:
            raise ValueError(
                "GroundingDINO requires a text_prompt "
                "(e.g., text_prompt='person . car . dog')"
            )

        from PIL import Image as PILImage

        from app.inference.results import (
            BoundingBox,
            Detection,
            DetectionResponse,
            ImageSize,
            InferenceMetadata,
        )

        img = _load_image_from_bytes(image_bytes)
        pil_img = PILImage.fromarray(cv2.cvtColor(img, cv2.COLOR_BGR2RGB))
        image_height, image_width = img.shape[:2]

        loop = asyncio.get_event_loop()

        def _run_inference():
            inputs = self._processor(
                images=pil_img, text=text_prompt, return_tensors="pt"
            )
            import torch

            inputs = {k: v.to(self._model.device) for k, v in inputs.items()}
            with torch.no_grad():
                outputs = self._model(**inputs)

            target_sizes = torch.tensor([[pil_img.height, pil_img.width]])
            results = self._processor.post_process_grounded_object_detection(
                outputs,
                inputs["input_ids"],
                box_threshold=confidence_threshold,
                text_threshold=confidence_threshold,
                target_sizes=target_sizes,
            )[0]
            return results

        results = await loop.run_in_executor(None, _run_inference)

        detections: list[Detection] = []
        boxes = results["boxes"].cpu().numpy()
        scores = results["scores"].cpu().numpy()
        labels = results["labels"]

        for i in range(len(boxes)):
            x1, y1, x2, y2 = boxes[i].tolist()
            conf = float(scores[i])
            class_name = str(labels[i]) if i < len(labels) else "unknown"

            detections.append(
                Detection(
                    class_id=i,
                    class_name=class_name,
                    confidence=round(conf, 6),
                    bbox=BoundingBox.from_xyxy(
                        x1=max(0.0, x1),
                        y1=max(0.0, y1),
                        x2=min(float(image_width), x2),
                        y2=min(float(image_height), y2),
                    ),
                )
            )

        return DetectionResponse(
            detections=detections,
            detection_count=len(detections),
            image_size=ImageSize(width=image_width, height=image_height),
            processing_time_ms=0.0,
            inference_time_ms=0.0,
            metadata=InferenceMetadata(
                model_name=self.name,
                image_size=ImageSize(width=image_width, height=image_height),
                source=source,
                confidence_threshold=confidence_threshold,
                iou_threshold=iou_threshold,
            ),
        )

    async def load_model(self) -> None:
        """Load the Grounding DINO model from HuggingFace Transformers."""
        from transformers import AutoModelForZeroShotObjectDetection, AutoProcessor

        model_id = self._resolve_model_id()
        logger.info(
            "Loading Grounding DINO model: %s from %s (device=%s)",
            self.name,
            model_id,
            self._metadata.device,
        )

        loop = asyncio.get_event_loop()

        def _load():
            processor = AutoProcessor.from_pretrained(model_id)
            model = AutoModelForZeroShotObjectDetection.from_pretrained(model_id)
            model = model.to(self._metadata.device)
            model.eval()
            return processor, model

        self._processor, self._model = await loop.run_in_executor(None, _load)

        self._metadata.input_shape = [1, 3, 800, 800]
        self._metadata.class_count = None
        self._metadata.extra = {
            "framework": "transformers",
            "task": "open-set-detect",
            "model_id": model_id,
        }

        logger.info("Grounding DINO model %s loaded from %s", self.name, model_id)

    async def unload_model(self) -> None:
        """Unload the Grounding DINO model and release resources."""
        logger.info("Unloading Grounding DINO model: %s", self.name)
        self._processor = None
        self._model = None
        self._metadata.input_shape = None
        self._metadata.class_count = None
        self._metadata.extra = {}
