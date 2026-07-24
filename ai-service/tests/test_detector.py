"""Tests for YoloDetector."""

import numpy as np
import pytest
from unittest.mock import MagicMock

from app.config import Settings
from app.inference.detector import YoloDetector


@pytest.fixture
def settings():
    """Create test settings."""
    return Settings(
        CONFIDENCE_THRESHOLD=0.5,
        IOU_THRESHOLD=0.45,
        INFERENCE_TIMEOUT=5.0,
    )


@pytest.fixture
def detector(settings):
    """Create a YoloDetector with test settings."""
    return YoloDetector(settings=settings)


def _make_mock_yolo_result(
    xyxy: np.ndarray,
    conf: np.ndarray,
    cls: np.ndarray,
    names: dict | None = None,
):
    """Create a mock Ultralytics Result with proper numpy data."""
    result = MagicMock()
    boxes = MagicMock()
    boxes.__len__ = MagicMock(return_value=len(xyxy))

    xyxy_mock = MagicMock()
    xyxy_mock.cpu.return_value.numpy.return_value = xyxy
    boxes.xyxy = xyxy_mock

    conf_mock = MagicMock()
    conf_mock.cpu.return_value.numpy.return_value = conf
    boxes.conf = conf_mock

    cls_mock = MagicMock()
    cls_mock.cpu.return_value.numpy.return_value = cls
    boxes.cls = cls_mock

    result.boxes = boxes
    result.names = names or {0: "person", 1: "car"}
    return result


class TestYoloDetectorDetect:
    """Tests for YoloDetector.detect."""

    @pytest.mark.asyncio
    async def test_detect_valid_image(self, detector, sample_image_bytes):
        """Test detection on a valid JPEG image."""
        mock_result = _make_mock_yolo_result(
            xyxy=np.array([[100, 100, 200, 200]], dtype=np.float32),
            conf=np.array([0.95], dtype=np.float32),
            cls=np.array([0], dtype=np.float32),
            names={0: "person"},
        )

        mock_model = MagicMock()
        mock_model.predict.return_value = [mock_result]

        response = await detector.detect(
            image_bytes=sample_image_bytes,
            model_name="yolov8n",
            model=mock_model,
            source="test.jpg",
            confidence_threshold=0.5,
            iou_threshold=0.45,
        )

        assert response.detection_count == 1
        assert len(response.detections) == 1
        assert response.detections[0].class_name == "person"
        assert response.detections[0].confidence == 0.95
        assert response.image_size.width > 0
        assert response.image_size.height > 0
        assert response.processing_time_ms > 0
        assert response.inference_time_ms >= 0

    @pytest.mark.asyncio
    async def test_detect_empty_image_raises(self, detector):
        """Test detection with empty bytes raises ValueError."""
        mock_model = MagicMock()

        with pytest.raises(ValueError, match="empty"):
            await detector.detect(
                image_bytes=b"",
                model_name="yolov8n",
                model=mock_model,
            )

    @pytest.mark.asyncio
    async def test_detect_corrupt_image_raises(self, detector, corrupt_image_bytes):
        """Test detection with corrupt image raises error."""
        mock_model = MagicMock()

        with pytest.raises((ValueError, RuntimeError)):
            await detector.detect(
                image_bytes=corrupt_image_bytes,
                model_name="yolov8n",
                model=mock_model,
            )

    @pytest.mark.asyncio
    async def test_detect_returns_zero_detections(self, detector, sample_image_bytes):
        """Test detection returns zero when nothing detected."""
        mock_result = _make_mock_yolo_result(
            xyxy=np.zeros((0, 4), dtype=np.float32),
            conf=np.zeros((0,), dtype=np.float32),
            cls=np.zeros((0,), dtype=np.float32),
        )

        mock_model = MagicMock()
        mock_model.predict.return_value = [mock_result]

        response = await detector.detect(
            image_bytes=sample_image_bytes,
            model_name="yolov8n",
            model=mock_model,
        )

        assert response.detection_count == 0
        assert response.detections == []

    @pytest.mark.asyncio
    async def test_detect_confidence_filtering(self, detector, sample_image_bytes):
        """Test that low-confidence detections are filtered out."""
        mock_result = _make_mock_yolo_result(
            xyxy=np.array(
                [[10, 10, 50, 50], [100, 100, 200, 200]],
                dtype=np.float32,
            ),
            conf=np.array([0.2, 0.9], dtype=np.float32),
            cls=np.array([0, 1], dtype=np.float32),
            names={0: "person", 1: "car"},
        )

        mock_model = MagicMock()
        mock_model.predict.return_value = [mock_result]

        response = await detector.detect(
            image_bytes=sample_image_bytes,
            model_name="yolov8n",
            model=mock_model,
            confidence_threshold=0.5,
        )

        assert response.detection_count == 1
        assert response.detections[0].confidence == 0.9
        assert response.detections[0].class_name == "car"

    @pytest.mark.asyncio
    async def test_detect_timeout(self, detector, sample_image_bytes):
        """Test inference timeout raises RuntimeError."""
        import time

        def slow_predict(**kwargs):
            time.sleep(10)
            return []

        mock_model = MagicMock()
        mock_model.predict.side_effect = slow_predict

        detector._settings = Settings(
            CONFIDENCE_THRESHOLD=0.5,
            IOU_THRESHOLD=0.45,
            INFERENCE_TIMEOUT=0.01,
        )

        with pytest.raises(RuntimeError, match="timed out"):
            await detector.detect(
                image_bytes=sample_image_bytes,
                model_name="yolov8n",
                model=mock_model,
            )

    @pytest.mark.asyncio
    async def test_detect_inference_failure(self, detector, sample_image_bytes):
        """Test inference failure raises RuntimeError."""
        mock_model = MagicMock()
        mock_model.predict.side_effect = RuntimeError("GPU out of memory")

        with pytest.raises(RuntimeError, match="Inference failed"):
            await detector.detect(
                image_bytes=sample_image_bytes,
                model_name="yolov8n",
                model=mock_model,
            )

    @pytest.mark.asyncio
    async def test_detect_includes_metadata(self, detector, sample_image_bytes):
        """Test detection response includes correct metadata."""
        mock_result = _make_mock_yolo_result(
            xyxy=np.zeros((0, 4), dtype=np.float32),
            conf=np.zeros((0,), dtype=np.float32),
            cls=np.zeros((0,), dtype=np.float32),
        )

        mock_model = MagicMock()
        mock_model.predict.return_value = [mock_result]

        response = await detector.detect(
            image_bytes=sample_image_bytes,
            model_name="yolov8n",
            model=mock_model,
            source="camera_01",
            confidence_threshold=0.3,
            iou_threshold=0.5,
        )

        assert response.metadata.model_name == "yolov8n"
        assert response.metadata.source == "camera_01"
        assert response.metadata.confidence_threshold == 0.3
        assert response.metadata.iou_threshold == 0.5
