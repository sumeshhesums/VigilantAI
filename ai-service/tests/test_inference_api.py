"""Tests for inference API endpoint."""

import numpy as np
import cv2
import pytest
from fastapi.testclient import TestClient


@pytest.fixture
def client(monkeypatch):
    """Create test client with lifespan (auto-load disabled)."""
    monkeypatch.setenv("AI_SERVICE_AUTO_LOAD", "false")
    from app.main import app

    with TestClient(app) as c:
        yield c


@pytest.fixture
def sample_jpeg():
    """Create a sample JPEG image as bytes."""
    img = np.random.randint(0, 255, (480, 640, 3), dtype=np.uint8)
    _, encoded = cv2.imencode(".jpg", img)
    return encoded.tobytes()


class TestInferenceEndpoint:
    """Tests for POST /inference endpoint."""

    def test_inference_returns_503_without_model(self, client, sample_jpeg):
        """Test inference returns 503 when no model is loaded."""
        response = client.post(
            "/inference",
            files={"file": ("test.jpg", sample_jpeg, "image/jpeg")},
        )
        assert response.status_code == 503

    def test_inference_rejects_unsupported_content_type(self, client):
        """Test inference rejects non-image content type."""
        response = client.post(
            "/inference",
            files={"file": ("test.txt", b"hello", "text/plain")},
        )
        assert response.status_code == 400
        assert "Unsupported content type" in response.json()["detail"]

    def test_inference_rejects_empty_file(self, client):
        """Test inference rejects empty file upload."""
        response = client.post(
            "/inference",
            files={"file": ("empty.jpg", b"", "image/jpeg")},
        )
        assert response.status_code == 400
        assert "empty" in response.json()["detail"].lower()

    def test_inference_with_confidence_threshold(self, client, sample_jpeg):
        """Test inference accepts confidence threshold parameter."""
        response = client.post(
            "/inference",
            files={"file": ("test.jpg", sample_jpeg, "image/jpeg")},
            data={"confidence_threshold": "0.3"},
        )
        assert response.status_code == 503

    def test_inference_with_iou_threshold(self, client, sample_jpeg):
        """Test inference accepts IoU threshold parameter."""
        response = client.post(
            "/inference",
            files={"file": ("test.jpg", sample_jpeg, "image/jpeg")},
            data={"iou_threshold": "0.6"},
        )
        assert response.status_code == 503


class TestInferenceAPIContracts:
    """Tests for API contract validation."""

    def test_inference_requires_file(self, client):
        """Test inference endpoint requires file upload."""
        response = client.post("/inference")
        assert response.status_code == 422

    def test_openapi_schema_has_inference_path(self, client):
        """Test OpenAPI schema includes inference endpoint."""
        response = client.get("/openapi.json")
        schema = response.json()
        assert "/inference" in schema["paths"]
        post_spec = schema["paths"]["/inference"]["post"]
        assert "requestBody" in post_spec
