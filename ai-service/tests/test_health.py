"""Tests for health endpoints."""

import pytest
from fastapi.testclient import TestClient


@pytest.fixture
def client(monkeypatch):
    """Create test client with lifespan (auto-load disabled)."""
    monkeypatch.setenv("AI_SERVICE_AUTO_LOAD", "false")
    from app.main import app

    with TestClient(app) as c:
        yield c


class TestHealthEndpoints:
    """Tests for health check endpoints."""

    def test_health_returns_200(self, client: TestClient):
        """Test health endpoint returns 200."""
        response = client.get("/health")
        assert response.status_code == 200

    def test_health_returns_correct_schema(self, client: TestClient):
        """Test health endpoint returns correct response schema."""
        response = client.get("/health")
        data = response.json()
        assert "status" in data
        assert "version" in data
        assert "uptime_seconds" in data
        assert "model" in data

    def test_health_status_is_healthy(self, client: TestClient):
        """Test health endpoint reports healthy status."""
        response = client.get("/health")
        data = response.json()
        assert data["status"] == "healthy"

    def test_health_detailed_returns_200(self, client: TestClient):
        """Test detailed health endpoint returns 200."""
        response = client.get("/health/detailed")
        assert response.status_code == 200

    def test_health_detailed_has_metrics(self, client: TestClient):
        """Test detailed health endpoint includes request metrics."""
        response = client.get("/health/detailed")
        data = response.json()
        assert "request_count" in data
        assert "successful_requests" in data
        assert "failed_requests" in data
        assert "average_inference_time_ms" in data
        assert "images_processed" in data
        assert "total_detections" in data
        assert "average_detections_per_image" in data

    def test_health_model_info_has_required_fields(self, client: TestClient):
        """Test health endpoint model info has required fields."""
        response = client.get("/health")
        data = response.json()
        model = data["model"]
        assert "name" in model
        assert "version" in model
        assert "status" in model
        assert "device" in model
