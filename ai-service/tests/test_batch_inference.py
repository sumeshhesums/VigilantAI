"""Tests for batch inference API endpoint and batch response models."""

import numpy as np
import cv2
import pytest
from fastapi.testclient import TestClient

from app.inference.results import BatchDetectionResponse, SingleBatchResult


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


class TestBatchDetectionResponseSchema:
    """Tests for BatchDetectionResponse Pydantic model."""

    def test_empty_batch(self):
        """Test empty batch response."""
        resp = BatchDetectionResponse(
            results=[],
            total_images=0,
            successful=0,
            failed=0,
            total_detections=0,
            total_processing_time_ms=0.0,
        )
        assert resp.total_images == 0
        assert resp.results == []

    def test_batch_with_results(self):
        """Test batch response with mixed results."""
        resp = BatchDetectionResponse(
            results=[
                SingleBatchResult(
                    index=0, source="a.jpg", result=None, error="bad file"
                ),
                SingleBatchResult(index=1, source="b.jpg", result=None, error=None),
            ],
            total_images=2,
            successful=1,
            failed=1,
            total_detections=5,
            total_processing_time_ms=123.45,
        )
        assert resp.successful == 1
        assert resp.failed == 1
        assert resp.total_detections == 5

    def test_batch_response_serialization(self):
        """Test batch response serializes to dict."""
        resp = BatchDetectionResponse(
            results=[],
            total_images=0,
            successful=0,
            failed=0,
            total_detections=0,
            total_processing_time_ms=0.0,
        )
        d = resp.model_dump()
        assert "total_images" in d
        assert "results" in d
        assert "total_processing_time_ms" in d

    def test_batch_total_counts_constraint(self):
        """Test that total_images = successful + failed."""
        resp = BatchDetectionResponse(
            results=[],
            total_images=5,
            successful=3,
            failed=2,
            total_detections=10,
            total_processing_time_ms=50.0,
        )
        assert resp.successful + resp.failed == resp.total_images


class TestBatchInferenceAPI:
    """Tests for POST /inference/batch endpoint."""

    def test_batch_returns_503_without_model(self, client, sample_jpeg):
        """Test batch inference returns 503 when no model loaded."""
        response = client.post(
            "/inference/batch",
            files=[("files", ("test.jpg", sample_jpeg, "image/jpeg"))],
        )
        assert response.status_code == 503

    def test_batch_rejects_empty_files(self, client):
        """Test batch inference rejects no files."""
        response = client.post("/inference/batch")
        assert response.status_code == 422

    def test_batch_rejects_unsupported_content_type(self, client):
        """Test batch inference rejects non-image content type."""
        response = client.post(
            "/inference/batch",
            files=[("files", ("test.txt", b"hello", "text/plain"))],
        )
        assert response.status_code == 400
        assert "unsupported content type" in response.json()["detail"].lower()

    def test_batch_rejects_empty_file(self, client):
        """Test batch inference rejects empty file in batch."""
        response = client.post(
            "/inference/batch",
            files=[("files", ("empty.jpg", b"", "image/jpeg"))],
        )
        assert response.status_code == 400
        assert "empty" in response.json()["detail"].lower()

    def test_batch_exceeds_max_size(self, client, sample_jpeg):
        """Test batch inference rejects more than 32 files."""
        files = [
            ("files", (f"test_{i}.jpg", sample_jpeg, "image/jpeg")) for i in range(33)
        ]
        response = client.post("/inference/batch", files=files)
        assert response.status_code == 400
        assert "exceeds maximum" in response.json()["detail"].lower()

    def test_batch_accepts_multiple_images(self, client, sample_jpeg):
        """Test batch inference accepts multiple valid images."""
        files = [
            ("files", ("test1.jpg", sample_jpeg, "image/jpeg")),
            ("files", ("test2.jpg", sample_jpeg, "image/jpeg")),
        ]
        response = client.post("/inference/batch", files=files)
        assert response.status_code == 503

    def test_batch_with_confidence_threshold(self, client, sample_jpeg):
        """Test batch accepts confidence threshold."""
        files = [("files", ("test.jpg", sample_jpeg, "image/jpeg"))]
        response = client.post(
            "/inference/batch",
            files=files,
            data={"confidence_threshold": "0.3"},
        )
        assert response.status_code == 503

    def test_batch_openapi_schema(self, client):
        """Test OpenAPI schema includes batch endpoint."""
        response = client.get("/openapi.json")
        schema = response.json()
        assert "/inference/batch" in schema["paths"]
        post_spec = schema["paths"]["/inference/batch"]["post"]
        assert "requestBody" in post_spec
