"""Tests for Pydantic schemas."""

import pytest
from pydantic import ValidationError

from app.schemas.detection import (
    BoundingBox,
    BatchDetectionRequest,
    Detection,
    DetectionRequest,
)
from app.schemas.health import (
    DetailedHealthResponse,
    HealthResponse,
    ModelInfo,
    ModelStatus,
    ServiceStatus,
)


class TestBoundingBox:
    """Tests for BoundingBox schema."""

    def test_valid_bounding_box(self):
        """Test creating valid bounding box."""
        bbox = BoundingBox(x_min=0, y_min=0, x_max=100, y_max=100)
        assert bbox.x_min == 0
        assert bbox.x_max == 100

    def test_invalid_negative_coordinates(self):
        """Test bounding box with negative coordinates fails."""
        with pytest.raises(ValidationError):
            BoundingBox(x_min=-1, y_min=0, x_max=100, y_max=100)


class TestDetection:
    """Tests for Detection schema."""

    def test_valid_detection(self):
        """Test creating valid detection."""
        detection = Detection(
            class_name="person",
            confidence=0.95,
            bbox=BoundingBox(x_min=0, y_min=0, x_max=100, y_max=100),
        )
        assert detection.class_name == "person"
        assert detection.confidence == 0.95

    def test_invalid_confidence(self):
        """Test detection with invalid confidence fails."""
        with pytest.raises(ValidationError):
            Detection(
                class_name="person",
                confidence=1.5,
                bbox=BoundingBox(x_min=0, y_min=0, x_max=100, y_max=100),
            )


class TestDetectionRequest:
    """Tests for DetectionRequest schema."""

    def test_valid_request(self):
        """Test creating valid detection request."""
        request = DetectionRequest(image_url="http://example.com/image.jpg")
        assert request.image_url == "http://example.com/image.jpg"
        assert request.confidence_threshold == 0.5

    def test_request_with_optional_fields(self):
        """Test request with optional fields."""
        request = DetectionRequest(
            image_url="http://example.com/image.jpg",
            camera_id="cam-001",
            confidence_threshold=0.7,
        )
        assert request.camera_id == "cam-001"
        assert request.confidence_threshold == 0.7


class TestBatchDetectionRequest:
    """Tests for BatchDetectionRequest schema."""

    def test_valid_batch_request(self):
        """Test creating valid batch request."""
        request = BatchDetectionRequest(
            requests=[
                DetectionRequest(image_url="http://example.com/image1.jpg"),
                DetectionRequest(image_url="http://example.com/image2.jpg"),
            ]
        )
        assert len(request.requests) == 2

    def test_empty_batch_request_fails(self):
        """Test empty batch request fails."""
        with pytest.raises(ValidationError):
            BatchDetectionRequest(requests=[])


class TestHealthSchemas:
    """Tests for health-related schemas."""

    def test_health_response(self):
        """Test HealthResponse schema."""
        response = HealthResponse(
            status=ServiceStatus.HEALTHY,
            version="0.1.0",
            uptime_seconds=100.0,
            model=ModelInfo(
                name="yolov8n",
                version="1.0.0",
                status=ModelStatus.NOT_LOADED,
                device="cpu",
            ),
        )
        assert response.status == ServiceStatus.HEALTHY

    def test_detailed_health_response(self):
        """Test DetailedHealthResponse schema."""
        response = DetailedHealthResponse(
            status=ServiceStatus.HEALTHY,
            version="0.1.0",
            uptime_seconds=100.0,
            model=ModelInfo(
                name="yolov8n",
                version="1.0.0",
                status=ModelStatus.NOT_LOADED,
                device="cpu",
            ),
            request_count=100,
            successful_requests=95,
            failed_requests=5,
            average_inference_time_ms=50.0,
        )
        assert response.request_count == 100
