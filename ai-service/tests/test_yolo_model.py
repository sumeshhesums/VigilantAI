"""Tests for YOLO model implementation."""

from unittest.mock import MagicMock, patch

import numpy as np
import pytest

from app.models.base import ModelState
from app.models.factory import YOLOModel


class TestYOLOModelLoad:
    """Tests for YOLO model loading."""

    @pytest.mark.asyncio
    async def test_load_sets_metadata(self):
        """Test loading sets correct metadata fields."""
        mock_model = MagicMock()
        mock_model.names = {0: "person", 1: "car", 2: "dog"}

        with patch("ultralytics.YOLO", return_value=mock_model):
            model = YOLOModel(name="yolov8n", device="cpu")
            await model.load()

        assert model.is_loaded
        assert model.metadata.input_shape == [1, 3, 640, 640]
        assert model.metadata.class_count == 3
        assert model.metadata.model_type == "yolo"
        assert model.metadata.extra["framework"] == "ultralytics"
        assert model.metadata.extra["names"] == {
            0: "person",
            1: "car",
            2: "dog",
        }

    @pytest.mark.asyncio
    async def test_load_sets_model_path(self):
        """Test loading with custom model path."""
        mock_model = MagicMock()
        mock_model.names = {0: "object"}

        with patch("ultralytics.YOLO", return_value=mock_model) as mock_yolo:
            model = YOLOModel(name="yolov8n", model_path="/custom/path.pt")
            await model.load()
            mock_yolo.assert_called_with("/custom/path.pt")

    @pytest.mark.asyncio
    async def test_load_default_path(self):
        """Test loading uses name.pt as default path."""
        mock_model = MagicMock()
        mock_model.names = {0: "object"}

        with patch("ultralytics.YOLO", return_value=mock_model) as mock_yolo:
            model = YOLOModel(name="yolov8n")
            await model.load()
            mock_yolo.assert_called_with("yolov8n.pt")

    @pytest.mark.asyncio
    async def test_load_idempotent(self):
        """Test loading already-loaded model is a no-op."""
        mock_model = MagicMock()
        mock_model.names = {0: "person"}

        with patch("ultralytics.YOLO", return_value=mock_model):
            model = YOLOModel(name="yolov8n")
            m1 = await model.load()
            m2 = await model.load()
            assert m1 is m2

    @pytest.mark.asyncio
    async def test_unload_clears_model(self):
        """Test unloading releases the YOLO model."""
        mock_model = MagicMock()
        mock_model.names = {0: "person"}

        with patch("ultralytics.YOLO", return_value=mock_model):
            model = YOLOModel(name="yolov8n")
            await model.load()
            assert model.yolo_model is not None

            await model.unload()
            assert model.yolo_model is None
            assert not model.is_loaded
            assert model.metadata.input_shape is None
            assert model.metadata.class_count is None


class TestYOLOModelPredict:
    """Tests for YOLO model prediction."""

    @pytest.mark.asyncio
    async def test_predict_returns_detection_response(self, sample_image_bytes):
        """Test predict returns a DetectionResponse."""
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
        assert response.detections[0].confidence == 0.95
        assert response.metadata.model_name == "yolov8n"

    @pytest.mark.asyncio
    async def test_predict_before_load_raises(self, sample_image_bytes):
        """Test predict before load raises RuntimeError."""
        model = YOLOModel(name="yolov8n")
        with pytest.raises(RuntimeError, match="not loaded"):
            await model.predict(image_bytes=sample_image_bytes)


class TestYOLOModelWarmup:
    """Tests for YOLO model warmup."""

    @pytest.mark.asyncio
    async def test_warmup_runs_dummy_inference(self):
        """Test warmup performs a dummy predict call."""
        mock_model = MagicMock()
        mock_model.names = {0: "person"}
        mock_model.predict.return_value = []

        with patch("ultralytics.YOLO", return_value=mock_model):
            model = YOLOModel(name="yolov8n")
            await model.load()
            result = await model.warmup()

            assert result.state == ModelState.LOADED
            assert mock_model.predict.called

    @pytest.mark.asyncio
    async def test_warmup_before_load_raises(self):
        """Test warmup before load raises RuntimeError."""
        model = YOLOModel(name="yolov8n")
        with pytest.raises(RuntimeError, match="must be loaded"):
            await model.warmup()


class TestYOLOModelHealth:
    """Tests for YOLO model health reporting."""

    def test_health_when_not_loaded(self):
        """Test health report when model is not loaded."""
        model = YOLOModel(name="yolov8n")
        health = model.health()
        assert health["name"] == "yolov8n"
        assert health["state"] == "not_loaded"
        assert health["loaded"] is False

    def test_health_when_loaded(self):
        """Test health report when model is loaded (sync check)."""
        model = YOLOModel(name="yolov8n")
        assert model.health()["loaded"] is False
