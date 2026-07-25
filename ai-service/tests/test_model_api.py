"""Tests for model management API endpoints."""

from unittest.mock import MagicMock, patch

import pytest
from fastapi.testclient import TestClient


@pytest.fixture
def client(monkeypatch):
    """Create test client with lifespan (auto-load disabled)."""
    monkeypatch.setenv("AI_SERVICE_AUTO_LOAD", "false")
    from app.main import app

    with TestClient(app) as c:
        yield c


class TestModelsListEndpoint:
    """Tests for GET /models endpoint."""

    def test_list_models_returns_200(self, client: TestClient):
        """Test list models endpoint returns 200."""
        response = client.get("/models")
        assert response.status_code == 200

    def test_list_models_has_correct_schema(self, client: TestClient):
        """Test list models response has correct fields."""
        response = client.get("/models")
        data = response.json()
        assert "models" in data
        assert "total" in data
        assert "active_model" in data

    def test_list_models_count_matches(self, client: TestClient):
        """Test total matches models list length."""
        response = client.get("/models")
        data = response.json()
        assert data["total"] == len(data["models"])

    def test_list_models_has_expected_models(self, client: TestClient):
        """Test list contains expected model names."""
        response = client.get("/models")
        data = response.json()
        names = [m["name"] for m in data["models"]]
        assert "yolov8n" in names
        assert "rtdetr-l" in names
        assert "grounding-dino-tiny" in names

    def test_list_models_active_model_set(self, client: TestClient):
        """Test active_model is set to default."""
        response = client.get("/models")
        data = response.json()
        assert data["active_model"] == "yolov8n"


class TestModelsGetEndpoint:
    """Tests for GET /models/{name} endpoint."""

    def test_get_model_returns_200(self, client: TestClient):
        """Test get model endpoint returns 200."""
        response = client.get("/models/yolov8n")
        assert response.status_code == 200

    def test_get_model_has_correct_schema(self, client: TestClient):
        """Test get model response has correct fields."""
        response = client.get("/models/yolov8n")
        data = response.json()
        assert data["name"] == "yolov8n"
        assert "version" in data
        assert "model_type" in data
        assert "state" in data
        assert "loaded" in data
        assert "is_active" in data

    def test_get_model_not_found(self, client: TestClient):
        """Test get nonexistent model returns 404."""
        response = client.get("/models/nonexistent")
        assert response.status_code == 404

    def test_get_model_is_active(self, client: TestClient):
        """Test default model is marked as active."""
        response = client.get("/models/yolov8n")
        data = response.json()
        assert data["is_active"] is True

    def test_get_model_other_not_active(self, client: TestClient):
        """Test non-default model is not marked as active."""
        response = client.get("/models/rtdetr-l")
        data = response.json()
        assert data["is_active"] is False


class TestModelsLoadEndpoint:
    """Tests for POST /models/{name}/load endpoint."""

    def test_load_rtdetr_model_returns_200(self, client: TestClient):
        """Test load RT-DETR model endpoint returns 200."""
        with patch("ultralytics.RTDETR", return_value=MagicMock()):
            response = client.post("/models/rtdetr-l/load")
        assert response.status_code == 200

    def test_load_rtdetr_model_response_schema(self, client: TestClient):
        """Test load RT-DETR model response has correct fields."""
        with patch("ultralytics.RTDETR", return_value=MagicMock()):
            response = client.post("/models/rtdetr-l/load")
        data = response.json()
        assert data["name"] == "rtdetr-l"
        assert data["action"] == "load"
        assert data["state"] == "loaded"

    def test_load_model_not_found(self, client: TestClient):
        """Test loading nonexistent model returns 404."""
        response = client.post("/models/nonexistent/load")
        assert response.status_code == 404

    def test_load_model_then_get_shows_loaded(self, client: TestClient):
        """Test loading model then getting it shows loaded state."""
        with patch("ultralytics.RTDETR", return_value=MagicMock()):
            client.post("/models/rtdetr-l/load")
        response = client.get("/models/rtdetr-l")
        data = response.json()
        assert data["loaded"] is True
        assert data["state"] == "loaded"


class TestModelsUnloadEndpoint:
    """Tests for POST /models/{name}/unload endpoint."""

    def test_unload_model_returns_200(self, client: TestClient):
        """Test unload model endpoint returns 200."""
        with patch("ultralytics.RTDETR", return_value=MagicMock()):
            client.post("/models/rtdetr-l/load")
        response = client.post("/models/rtdetr-l/unload")
        assert response.status_code == 200

    def test_unload_model_response_schema(self, client: TestClient):
        """Test unload model response has correct fields."""
        with patch("ultralytics.RTDETR", return_value=MagicMock()):
            client.post("/models/rtdetr-l/load")
        response = client.post("/models/rtdetr-l/unload")
        data = response.json()
        assert data["name"] == "rtdetr-l"
        assert data["action"] == "unload"
        assert data["state"] == "not_loaded"

    def test_unload_model_not_found(self, client: TestClient):
        """Test unloading nonexistent model returns 404."""
        response = client.post("/models/nonexistent/unload")
        assert response.status_code == 404


class TestModelsReloadEndpoint:
    """Tests for POST /models/{name}/reload endpoint."""

    def test_reload_model_returns_200(self, client: TestClient):
        """Test reload model endpoint returns 200."""
        with patch("ultralytics.RTDETR", return_value=MagicMock()):
            client.post("/models/rtdetr-l/load")
        with patch("ultralytics.RTDETR", return_value=MagicMock()):
            response = client.post("/models/rtdetr-l/reload")
        assert response.status_code == 200

    def test_reload_model_response_schema(self, client: TestClient):
        """Test reload model response has correct fields."""
        with patch("ultralytics.RTDETR", return_value=MagicMock()):
            client.post("/models/rtdetr-l/load")
        with patch("ultralytics.RTDETR", return_value=MagicMock()):
            response = client.post("/models/rtdetr-l/reload")
        data = response.json()
        assert data["name"] == "rtdetr-l"
        assert data["action"] == "reload"
        assert data["state"] == "loaded"

    def test_reload_model_not_found(self, client: TestClient):
        """Test reloading nonexistent model returns 404."""
        response = client.post("/models/nonexistent/reload")
        assert response.status_code == 404


class TestModelsOpenAPISchema:
    """Tests for model endpoints in OpenAPI schema."""

    def test_models_path_in_schema(self, client: TestClient):
        """Test /models path exists in OpenAPI."""
        response = client.get("/openapi.json")
        schema = response.json()
        assert "/models" in schema["paths"]

    def test_model_detail_path_in_schema(self, client: TestClient):
        """Test /models/{name} path exists in OpenAPI."""
        response = client.get("/openapi.json")
        schema = response.json()
        assert "/models/{name}" in schema["paths"]

    def test_model_load_path_in_schema(self, client: TestClient):
        """Test /models/{name}/load path exists in OpenAPI."""
        response = client.get("/openapi.json")
        schema = response.json()
        assert "/models/{name}/load" in schema["paths"]

    def test_model_unload_path_in_schema(self, client: TestClient):
        """Test /models/{name}/unload path exists in OpenAPI."""
        response = client.get("/openapi.json")
        schema = response.json()
        assert "/models/{name}/unload" in schema["paths"]

    def test_model_reload_path_in_schema(self, client: TestClient):
        """Test /models/{name}/reload path exists in OpenAPI."""
        response = client.get("/openapi.json")
        schema = response.json()
        assert "/models/{name}/reload" in schema["paths"]
