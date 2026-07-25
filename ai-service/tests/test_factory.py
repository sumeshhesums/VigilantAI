"""Tests for model factory implementations."""

from unittest.mock import MagicMock, patch

import numpy as np
import pytest

from app.models.base import ModelState
from app.models.factory import GroundingDINOModel, RTDETRModel, YOLOModel


class TestYOLOModel:
    """Tests for YOLOModel."""

    @pytest.mark.asyncio
    async def test_load_sets_metadata(self):
        """Test loading YOLO model sets correct metadata."""
        mock_model = MagicMock()
        mock_model.names = {0: "person", 1: "car", 2: "dog"}

        with patch("ultralytics.YOLO", return_value=mock_model):
            model = YOLOModel(name="yolov8n", version="8.0.0", device="cpu")
            await model.load()
        assert model.is_loaded
        assert model.metadata.input_shape == [1, 3, 640, 640]
        assert model.metadata.class_count == 3
        assert model.metadata.model_type == "yolo"
        assert model.metadata.extra["framework"] == "ultralytics"

    @pytest.mark.asyncio
    async def test_unload_clears_metadata(self):
        """Test unloading YOLO model clears metadata."""
        mock_model = MagicMock()
        mock_model.names = {0: "person"}

        with patch("ultralytics.YOLO", return_value=mock_model):
            model = YOLOModel()
            await model.load()
            await model.unload()
        assert not model.is_loaded
        assert model.metadata.input_shape is None
        assert model.metadata.class_count is None

    @pytest.mark.asyncio
    async def test_default_name_and_version(self):
        """Test default name and version."""
        model = YOLOModel()
        assert model.name == "yolov8n"
        assert model.metadata.version == "8.0.0"

    def test_health_report(self):
        """Test health report structure."""
        model = YOLOModel()
        health = model.health()
        assert "name" in health
        assert "state" in health
        assert "loaded" in health
        assert health["loaded"] is False

    @pytest.mark.asyncio
    async def test_predict_returns_detection_response(self, sample_image_bytes):
        """Test predict returns a proper DetectionResponse."""
        mock_result = MagicMock()
        mock_model = MagicMock()
        mock_model.names = {0: "person"}
        mock_model.predict.return_value = [mock_result]

        boxes = MagicMock()
        boxes.__len__ = MagicMock(return_value=1)
        xyxy_mock = MagicMock()
        xyxy_mock.cpu.return_value.numpy.return_value = np.array(
            [[100, 100, 200, 200]], dtype=np.float32
        )
        boxes.xyxy = xyxy_mock
        conf_mock = MagicMock()
        conf_mock.cpu.return_value.numpy.return_value = np.array(
            [0.95], dtype=np.float32
        )
        boxes.conf = conf_mock
        cls_mock = MagicMock()
        cls_mock.cpu.return_value.numpy.return_value = np.array([0], dtype=np.float32)
        boxes.cls = cls_mock
        mock_result.boxes = boxes
        mock_result.names = {0: "person"}

        with patch("ultralytics.YOLO", return_value=mock_model):
            model = YOLOModel(name="yolov8n")
            await model.load()
            response = await model.predict(
                image_bytes=sample_image_bytes,
                confidence_threshold=0.5,
                iou_threshold=0.45,
            )

        assert response.detection_count == 1
        assert response.detections[0].class_name == "person"
        assert response.metadata.model_name == "yolov8n"

    @pytest.mark.asyncio
    async def test_predict_before_load_raises(self, sample_image_bytes):
        """Test predict before load raises RuntimeError."""
        model = YOLOModel(name="yolov8n")
        with pytest.raises(RuntimeError, match="not loaded"):
            await model.predict(image_bytes=sample_image_bytes)


class TestRTDETRModel:
    """Tests for RTDETRModel."""

    @pytest.mark.asyncio
    async def test_load_sets_metadata(self):
        """Test loading RTDETR model sets correct metadata."""
        mock_model = MagicMock()
        mock_model.names = {0: "person", 1: "car", 2: "dog"}

        with patch("ultralytics.RTDETR", return_value=mock_model):
            model = RTDETRModel(name="rtdetr-l")
            await model.load()
        assert model.is_loaded
        assert model.metadata.input_shape == [1, 3, 640, 640]
        assert model.metadata.class_count == 3
        assert model.metadata.model_type == "rtdetr"
        assert model.metadata.extra["framework"] == "ultralytics"
        assert model.metadata.extra["architecture"] == "rtdetr"

    @pytest.mark.asyncio
    async def test_unload(self):
        """Test unloading RTDETR model."""
        mock_model = MagicMock()
        mock_model.names = {0: "person"}

        with patch("ultralytics.RTDETR", return_value=mock_model):
            model = RTDETRModel()
            await model.load()
            await model.unload()
        assert not model.is_loaded

    def test_default_name(self):
        """Test default name."""
        model = RTDETRModel()
        assert model.name == "rtdetr-l"

    @pytest.mark.asyncio
    async def test_predict_returns_detection_response(self, sample_image_bytes):
        """Test predict returns a proper DetectionResponse."""
        mock_result = MagicMock()
        mock_model = MagicMock()
        mock_model.names = {0: "person"}
        mock_model.predict.return_value = [mock_result]

        boxes = MagicMock()
        boxes.__len__ = MagicMock(return_value=1)
        xyxy_mock = MagicMock()
        xyxy_mock.cpu.return_value.numpy.return_value = np.array(
            [[50, 50, 150, 150]], dtype=np.float32
        )
        boxes.xyxy = xyxy_mock
        conf_mock = MagicMock()
        conf_mock.cpu.return_value.numpy.return_value = np.array(
            [0.88], dtype=np.float32
        )
        boxes.conf = conf_mock
        cls_mock = MagicMock()
        cls_mock.cpu.return_value.numpy.return_value = np.array([0], dtype=np.float32)
        boxes.cls = cls_mock
        mock_result.boxes = boxes
        mock_result.names = {0: "person"}

        with patch("ultralytics.RTDETR", return_value=mock_model):
            model = RTDETRModel(name="rtdetr-l")
            await model.load()
            response = await model.predict(
                image_bytes=sample_image_bytes,
                confidence_threshold=0.5,
                iou_threshold=0.45,
            )

        assert response.detection_count == 1
        assert response.detections[0].class_name == "person"
        assert response.metadata.model_name == "rtdetr-l"

    @pytest.mark.asyncio
    async def test_predict_before_load_raises(self, sample_image_bytes):
        """Test predict before load raises RuntimeError."""
        model = RTDETRModel(name="rtdetr-l")
        with pytest.raises(RuntimeError, match="not loaded"):
            await model.predict(image_bytes=sample_image_bytes)

    @pytest.mark.asyncio
    async def test_warmup_after_load(self):
        """Test warmup works after loading."""
        mock_model = MagicMock()
        mock_model.names = {0: "person"}
        mock_model.predict.return_value = []

        with patch("ultralytics.RTDETR", return_value=mock_model):
            model = RTDETRModel()
            await model.load()
            result = await model.warmup()
            assert result.state == ModelState.LOADED


class TestGroundingDINOModel:
    """Tests for GroundingDINOModel."""

    @pytest.mark.asyncio
    async def test_load_sets_metadata(self):
        """Test loading Grounding DINO model sets correct metadata."""
        mock_processor = MagicMock()
        mock_model = MagicMock()
        mock_model.device = MagicMock()

        with patch(
            "transformers.AutoProcessor",
        ) as mock_auto_proc, patch(
            "transformers.AutoModelForZeroShotObjectDetection",
        ) as mock_auto_model:
            mock_auto_proc.from_pretrained.return_value = mock_processor
            mock_auto_model.from_pretrained.return_value = mock_model
            model = GroundingDINOModel()
            await model.load()
        assert model.is_loaded
        assert model.metadata.input_shape == [1, 3, 800, 800]
        assert model.metadata.class_count is None
        assert model.metadata.model_type == "grounding_dino"
        assert model.metadata.extra["task"] == "open-set-detect"

    @pytest.mark.asyncio
    async def test_unload(self):
        """Test unloading Grounding DINO model."""
        mock_processor = MagicMock()
        mock_model = MagicMock()
        mock_model.device = MagicMock()

        with patch(
            "transformers.AutoProcessor",
        ) as mock_auto_proc, patch(
            "transformers.AutoModelForZeroShotObjectDetection",
        ) as mock_auto_model:
            mock_auto_proc.from_pretrained.return_value = mock_processor
            mock_auto_model.from_pretrained.return_value = mock_model
            model = GroundingDINOModel()
            await model.load()
            await model.unload()
        assert not model.is_loaded

    def test_default_name(self):
        """Test default name."""
        model = GroundingDINOModel()
        assert model.name == "grounding-dino-tiny"

    @pytest.mark.asyncio
    async def test_predict_requires_text_prompt(self, sample_image_bytes):
        """Test predict raises ValueError without text_prompt."""
        mock_processor = MagicMock()
        mock_model = MagicMock()
        mock_model.device = MagicMock()

        with patch(
            "transformers.AutoProcessor",
        ) as mock_auto_proc, patch(
            "transformers.AutoModelForZeroShotObjectDetection",
        ) as mock_auto_model:
            mock_auto_proc.from_pretrained.return_value = mock_processor
            mock_auto_model.from_pretrained.return_value = mock_model
            model = GroundingDINOModel()
            await model.load()
            with pytest.raises(ValueError, match="text_prompt"):
                await model.predict(image_bytes=sample_image_bytes)

    @pytest.mark.asyncio
    async def test_predict_before_load_raises(self, sample_image_bytes):
        """Test predict before load raises RuntimeError."""
        model = GroundingDINOModel()
        with pytest.raises(RuntimeError, match="not loaded"):
            await model.predict(
                image_bytes=sample_image_bytes,
                text_prompt="person",
            )

    @pytest.mark.asyncio
    async def test_predict_returns_detection_response(self, sample_image_bytes):
        """Test predict returns a proper DetectionResponse."""
        import torch

        mock_processor = MagicMock()
        mock_model_inner = MagicMock()
        mock_model_inner.device = torch.device("cpu")
        mock_model_inner.to.return_value = mock_model_inner
        mock_model_inner.eval.return_value = mock_model_inner

        mock_outputs = MagicMock()
        mock_processor.return_value = {"input_ids": torch.tensor([[1]])}
        mock_model_inner.return_value = mock_outputs

        mock_results = {
            "boxes": torch.tensor([[100, 100, 200, 200]]),
            "scores": torch.tensor([0.95]),
            "labels": ["person"],
        }
        mock_processor.post_process_grounded_object_detection.return_value = [
            mock_results
        ]

        model = GroundingDINOModel(name="grounding-dino-tiny", device="cpu")
        model._processor = mock_processor
        model._model = mock_model_inner
        model._metadata.state = ModelState.LOADED
        model._metadata.input_shape = [1, 3, 800, 800]
        model._metadata.extra = {"task": "open-set-detect"}

        response = await model.predict(
            image_bytes=sample_image_bytes,
            confidence_threshold=0.5,
            text_prompt="person",
        )

        assert response.detection_count == 1
        assert response.detections[0].class_name == "person"
        assert response.metadata.model_name == "grounding-dino-tiny"


class TestBaseModelLifecycle:
    """Tests for BaseModel lifecycle via concrete classes."""

    @pytest.mark.asyncio
    async def test_load_idempotent(self):
        """Test loading already loaded model returns immediately."""
        mock_model = MagicMock()
        mock_model.names = {0: "person"}

        with patch("ultralytics.YOLO", return_value=mock_model):
            model = YOLOModel()
            m1 = await model.load()
            m2 = await model.load()
            assert m1 is m2

    @pytest.mark.asyncio
    async def test_warmup_after_load(self):
        """Test warmup works after loading."""
        mock_model = MagicMock()
        mock_model.names = {0: "person"}
        mock_model.predict.return_value = []

        with patch("ultralytics.YOLO", return_value=mock_model):
            model = YOLOModel()
            await model.load()
            result = await model.warmup()
            assert result.state == ModelState.LOADED

    @pytest.mark.asyncio
    async def test_warmup_before_load_raises(self):
        """Test warmup before load raises RuntimeError."""
        model = YOLOModel()
        with pytest.raises(RuntimeError, match="must be loaded"):
            await model.warmup()

    @pytest.mark.asyncio
    async def test_unload_when_not_loaded_is_safe(self):
        """Test unloading when not loaded is a no-op."""
        model = YOLOModel()
        await model.unload()
        assert not model.is_loaded
