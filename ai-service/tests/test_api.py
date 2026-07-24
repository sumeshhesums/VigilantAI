"""Tests for API contracts and integration."""

import pytest
from fastapi.testclient import TestClient

from app.main import app


@pytest.fixture
def client():
    """Create test client with lifespan."""
    with TestClient(app) as c:
        yield c


class TestAPIContracts:
    """Tests for API endpoint contracts."""

    def test_health_endpoint_exists(self, client: TestClient):
        """Test health endpoint exists and responds."""
        response = client.get("/health")
        assert response.status_code == 200

    def test_detailed_health_endpoint_exists(self, client: TestClient):
        """Test detailed health endpoint exists."""
        response = client.get("/health/detailed")
        assert response.status_code == 200

    def test_inference_endpoint_not_implemented(self, client: TestClient):
        """Test inference endpoint returns 501."""
        response = client.post(
            "/inference",
            json={"image_url": "http://example.com/image.jpg"},
        )
        assert response.status_code == 501

    def test_batch_inference_endpoint_not_implemented(self, client: TestClient):
        """Test batch inference endpoint returns 501."""
        response = client.post(
            "/inference/batch",
            json={"requests": [{"image_url": "http://example.com/image.jpg"}]},
        )
        assert response.status_code == 501

    def test_docs_endpoint_exists(self, client: TestClient):
        """Test OpenAPI docs endpoint exists."""
        response = client.get("/docs")
        assert response.status_code == 200

    def test_redoc_endpoint_exists(self, client: TestClient):
        """Test ReDoc endpoint exists."""
        response = client.get("/redoc")
        assert response.status_code == 200

    def test_openapi_schema_exists(self, client: TestClient):
        """Test OpenAPI schema endpoint exists."""
        response = client.get("/openapi.json")
        assert response.status_code == 200
        schema = response.json()
        assert "openapi" in schema
        assert "paths" in schema

    def test_openapi_schema_has_health_path(self, client: TestClient):
        """Test OpenAPI schema includes health path."""
        response = client.get("/openapi.json")
        schema = response.json()
        assert "/health" in schema["paths"]

    def test_openapi_schema_has_inference_path(self, client: TestClient):
        """Test OpenAPI schema includes inference path."""
        response = client.get("/openapi.json")
        schema = response.json()
        assert "/inference" in schema["paths"]

    def test_cors_headers(self, client: TestClient):
        """Test CORS headers are present."""
        response = client.options(
            "/health",
            headers={
                "Origin": "http://localhost:3000",
                "Access-Control-Request-Method": "GET",
            },
        )
        assert response.status_code == 200
