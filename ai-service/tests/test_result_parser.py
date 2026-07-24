"""Tests for ResultParser."""

import numpy as np
import pytest
from unittest.mock import MagicMock

from app.inference.parser import ResultParser
from app.inference.results import Detection, BoundingBox


@pytest.fixture
def parser():
    """Create a ResultParser."""
    return ResultParser()


def _make_mock_result(
    xyxy: np.ndarray,
    conf: np.ndarray,
    cls: np.ndarray,
    names: dict | None = None,
):
    """Create a mock Ultralytics result with proper numpy data."""
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


class TestResultParserParse:
    """Tests for ResultParser.parse."""

    def test_parse_single_detection(self, parser):
        """Test parsing a single detection."""
        result = _make_mock_result(
            xyxy=np.array([[100, 200, 300, 400]], dtype=np.float32),
            conf=np.array([0.95], dtype=np.float32),
            cls=np.array([0], dtype=np.float32),
        )

        response = parser.parse(
            results=[result],
            image_width=640,
            image_height=480,
            model_name="yolov8n",
            confidence_threshold=0.5,
            iou_threshold=0.45,
        )

        assert response.detection_count == 1
        assert response.detections[0].class_id == 0
        assert response.detections[0].class_name == "person"
        assert response.detections[0].confidence == 0.95
        assert response.detections[0].bbox.x1 == 100
        assert response.detections[0].bbox.y1 == 200
        assert response.detections[0].bbox.x2 == 300
        assert response.detections[0].bbox.y2 == 400

    def test_parse_multiple_detections(self, parser):
        """Test parsing multiple detections."""
        result = _make_mock_result(
            xyxy=np.array(
                [[10, 10, 50, 50], [100, 100, 200, 200], [300, 300, 400, 400]],
                dtype=np.float32,
            ),
            conf=np.array([0.9, 0.8, 0.7], dtype=np.float32),
            cls=np.array([0, 1, 2], dtype=np.float32),
            names={0: "person", 1: "car", 2: "dog"},
        )

        response = parser.parse(
            results=[result],
            image_width=640,
            image_height=480,
            model_name="yolov8n",
            confidence_threshold=0.5,
            iou_threshold=0.45,
        )

        assert response.detection_count == 3
        assert response.detections[0].class_name == "person"
        assert response.detections[1].class_name == "car"
        assert response.detections[2].class_name == "dog"

    def test_parse_filters_by_confidence(self, parser):
        """Test low-confidence detections are filtered."""
        result = _make_mock_result(
            xyxy=np.array(
                [[10, 10, 50, 50], [100, 100, 200, 200]],
                dtype=np.float32,
            ),
            conf=np.array([0.3, 0.9], dtype=np.float32),
            cls=np.array([0, 1], dtype=np.float32),
        )

        response = parser.parse(
            results=[result],
            image_width=640,
            image_height=480,
            model_name="yolov8n",
            confidence_threshold=0.5,
            iou_threshold=0.45,
        )

        assert response.detection_count == 1
        assert response.detections[0].confidence == 0.9

    def test_parse_empty_results(self, parser):
        """Test parsing empty results list."""
        response = parser.parse(
            results=[],
            image_width=640,
            image_height=480,
            model_name="yolov8n",
            confidence_threshold=0.5,
            iou_threshold=0.45,
        )

        assert response.detection_count == 0
        assert response.detections == []

    def test_parse_no_boxes(self, parser):
        """Test parsing result with no boxes."""
        result = MagicMock()
        result.boxes = None
        result.names = {}

        response = parser.parse(
            results=[result],
            image_width=640,
            image_height=480,
            model_name="yolov8n",
            confidence_threshold=0.5,
            iou_threshold=0.45,
        )

        assert response.detection_count == 0

    def test_parse_clips_boxes_to_image_bounds(self, parser):
        """Test that boxes are clipped to image dimensions."""
        result = _make_mock_result(
            xyxy=np.array([[-10, -5, 700, 500]], dtype=np.float32),
            conf=np.array([0.9], dtype=np.float32),
            cls=np.array([0], dtype=np.float32),
        )

        response = parser.parse(
            results=[result],
            image_width=640,
            image_height=480,
            model_name="yolov8n",
            confidence_threshold=0.5,
            iou_threshold=0.45,
        )

        bbox = response.detections[0].bbox
        assert bbox.x1 >= 0
        assert bbox.y1 >= 0
        assert bbox.x2 <= 640
        assert bbox.y2 <= 480

    def test_parse_metadata(self, parser):
        """Test that metadata is correctly set."""
        result = _make_mock_result(
            xyxy=np.zeros((0, 4), dtype=np.float32),
            conf=np.zeros((0,), dtype=np.float32),
            cls=np.zeros((0,), dtype=np.float32),
        )

        response = parser.parse(
            results=[result],
            image_width=1920,
            image_height=1080,
            model_name="yolov8s",
            confidence_threshold=0.3,
            iou_threshold=0.5,
            source="camera_05",
        )

        assert response.metadata.model_name == "yolov8s"
        assert response.metadata.image_size.width == 1920
        assert response.metadata.image_size.height == 1080
        assert response.metadata.source == "camera_05"
        assert response.metadata.confidence_threshold == 0.3
        assert response.metadata.iou_threshold == 0.5


class TestBoundingBoxFromXYXY:
    """Tests for BoundingBox.from_xyxy class method."""

    def test_from_xyxy_computes_derived_fields(self):
        """Test that from_xyxy computes width, height, center."""
        bbox = BoundingBox.from_xyxy(x1=10, y1=20, x2=110, y2=220)
        assert bbox.x1 == 10
        assert bbox.y1 == 20
        assert bbox.x2 == 110
        assert bbox.y2 == 220
        assert bbox.width == 100
        assert bbox.height == 200
        assert bbox.center_x == 60
        assert bbox.center_y == 120


class TestFilterByConfidence:
    """Tests for filter_by_confidence method."""

    def test_filter(self, parser):
        """Test confidence filtering."""
        d1 = Detection(
            class_id=0,
            class_name="a",
            confidence=0.3,
            bbox=BoundingBox.from_xyxy(0, 0, 10, 10),
        )
        d2 = Detection(
            class_id=1,
            class_name="b",
            confidence=0.9,
            bbox=BoundingBox.from_xyxy(0, 0, 10, 10),
        )
        filtered = parser.filter_by_confidence([d1, d2], threshold=0.5)
        assert len(filtered) == 1
        assert filtered[0].confidence == 0.9


class TestDetectionsToNumpy:
    """Tests for detections_to_numpy method."""

    def test_empty_detections(self, parser):
        """Test empty list returns empty array."""
        arr = parser.detections_to_numpy([])
        assert arr.shape == (0, 6)

    def test_converts_detections(self, parser):
        """Test detections are converted to NumPy array."""
        d = Detection(
            class_id=2,
            class_name="dog",
            confidence=0.85,
            bbox=BoundingBox.from_xyxy(10, 20, 100, 200),
        )
        arr = parser.detections_to_numpy([d])
        assert arr.shape == (1, 6)
        assert arr[0, 0] == 10  # x1
        assert arr[0, 1] == 20  # y1
        assert arr[0, 2] == 100  # x2
        assert arr[0, 3] == 200  # y2
        assert arr[0, 4] == pytest.approx(0.85)  # confidence
        assert arr[0, 5] == 2  # class_id
