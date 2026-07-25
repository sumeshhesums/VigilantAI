"""Tests for model manager."""

from unittest.mock import MagicMock, patch

import pytest

from app.core.model_manager import ModelManager
from app.models.base import ModelState
from app.models.factory import RTDETRModel
from app.models.loader import ModelLoader
from app.models.registry import ModelRegistry


@pytest.fixture
def manager():
    """Create a ModelManager with a single test model."""
    reg = ModelRegistry()
    reg.register_model(RTDETRModel(name="test_model", version="1.0.0"))
    reg.set_default("test_model")
    loader = ModelLoader(reg)
    return ModelManager(
        registry=reg,
        loader=loader,
        default_model_name="test_model",
    )


class TestModelManager:
    """Tests for ModelManager class."""

    @pytest.mark.asyncio
    async def test_initial_state_is_not_loaded(self, manager: ModelManager):
        """Test model starts in NOT_LOADED state."""
        assert manager.metadata.state == ModelState.NOT_LOADED
        assert not manager.is_loaded

    @pytest.mark.asyncio
    async def test_load_model_changes_state(self, manager: ModelManager):
        """Test loading model changes state to LOADED."""
        with patch("ultralytics.RTDETR", return_value=MagicMock()):
            await manager.load_by_name("test_model")
        assert manager.is_loaded

    @pytest.mark.asyncio
    async def test_load_model_sets_metadata(self, manager: ModelManager):
        """Test loading model sets metadata correctly."""
        mock_model = MagicMock()
        mock_model.names = {0: "person", 1: "car"}
        with patch("ultralytics.RTDETR", return_value=mock_model):
            await manager.load_by_name("test_model")
        info = manager.get_model_info()
        assert info["input_shape"] == [1, 3, 640, 640]
        assert info["class_count"] == 2

    @pytest.mark.asyncio
    async def test_load_already_loaded_model(self, manager: ModelManager):
        """Test loading already loaded model returns same metadata."""
        with patch("ultralytics.RTDETR", return_value=MagicMock()):
            await manager.load_by_name("test_model")
        with patch("ultralytics.RTDETR", return_value=MagicMock()):
            await manager.load_by_name("test_model")
        assert manager.is_loaded

    @pytest.mark.asyncio
    async def test_unload_model(self, manager: ModelManager):
        """Test unloading model changes state to NOT_LOADED."""
        with patch("ultralytics.RTDETR", return_value=MagicMock()):
            await manager.load_by_name("test_model")
        await manager.unload_by_name("test_model")
        assert manager.metadata.state == ModelState.NOT_LOADED
        assert not manager.is_loaded

    @pytest.mark.asyncio
    async def test_unload_not_loaded_model(self, manager: ModelManager):
        """Test unloading not loaded model is safe."""
        await manager.unload_by_name("test_model")
        assert manager.metadata.state == ModelState.NOT_LOADED

    @pytest.mark.asyncio
    async def test_reload_model(self, manager: ModelManager):
        """Test reloading model works correctly."""
        with patch("ultralytics.RTDETR", return_value=MagicMock()):
            await manager.load_by_name("test_model")
        with patch("ultralytics.RTDETR", return_value=MagicMock()):
            result = await manager.reload_by_name("test_model")
        assert result["state"] == "loaded"

    def test_get_status(self, manager: ModelManager):
        """Test get_status returns correct dictionary."""
        status = manager.get_status("test_model")
        assert "name" in status
        assert "state" in status
        assert status["name"] == "test_model"

    def test_get_model_info(self, manager: ModelManager):
        """Test get_model_info returns correct dictionary."""
        info = manager.get_model_info()
        assert "name" in info
        assert "version" in info
        assert "status" in info
        assert "device" in info
        assert info["name"] == "test_model"
        assert info["version"] == "1.0.0"

    def test_list_models(self, manager: ModelManager):
        """Test list_models returns registered models."""
        models = manager.list_models()
        assert isinstance(models, dict)
        assert len(models) == 1
        assert "test_model" in models

    def test_get_all_statuses(self, manager: ModelManager):
        """Test get_all_statuses returns all model statuses."""
        statuses = manager.get_all_statuses()
        assert len(statuses) == 1
        assert "test_model" in statuses
