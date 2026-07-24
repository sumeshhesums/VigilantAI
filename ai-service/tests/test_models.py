"""Tests for model manager."""

import pytest

from app.core.model_manager import ModelManager, ModelState


class TestModelManager:
    """Tests for ModelManager class."""

    @pytest.fixture
    def manager(self):
        """Create a ModelManager instance."""
        return ModelManager(
            model_name="test_model",
            model_version="1.0.0",
            device="cpu",
        )

    @pytest.mark.asyncio
    async def test_initial_state_is_not_loaded(self, manager: ModelManager):
        """Test model starts in NOT_LOADED state."""
        assert manager.metadata.state == ModelState.NOT_LOADED
        assert not manager.is_loaded

    @pytest.mark.asyncio
    async def test_load_model_changes_state(self, manager: ModelManager):
        """Test loading model changes state to LOADED."""
        await manager.load_model()
        assert manager.metadata.state == ModelState.LOADED
        assert manager.is_loaded

    @pytest.mark.asyncio
    async def test_load_model_sets_metadata(self, manager: ModelManager):
        """Test loading model sets metadata correctly."""
        await manager.load_model()
        assert manager.metadata.loaded_at is not None
        assert manager.metadata.load_duration_seconds is not None
        assert manager.metadata.input_shape == [1, 3, 640, 640]
        assert manager.metadata.class_count == 80

    @pytest.mark.asyncio
    async def test_load_already_loaded_model(self, manager: ModelManager):
        """Test loading already loaded model returns same metadata."""
        await manager.load_model()
        metadata1 = await manager.load_model()
        assert metadata1.state == ModelState.LOADED

    @pytest.mark.asyncio
    async def test_unload_model(self, manager: ModelManager):
        """Test unloading model changes state to NOT_LOADED."""
        await manager.load_model()
        await manager.unload_model()
        assert manager.metadata.state == ModelState.NOT_LOADED
        assert not manager.is_loaded

    @pytest.mark.asyncio
    async def test_unload_not_loaded_model(self, manager: ModelManager):
        """Test unloading not loaded model is safe."""
        await manager.unload_model()
        assert manager.metadata.state == ModelState.NOT_LOADED

    @pytest.mark.asyncio
    async def test_reload_model(self, manager: ModelManager):
        """Test reloading model works correctly."""
        await manager.load_model()
        metadata = await manager.reload_model()
        assert metadata.state == ModelState.LOADED

    def test_get_status(self, manager: ModelManager):
        """Test get_status returns correct dictionary."""
        status = manager.get_status()
        assert "name" in status
        assert "version" in status
        assert "status" in status
        assert "device" in status
        assert status["name"] == "test_model"
        assert status["version"] == "1.0.0"

    def test_get_model_info(self, manager: ModelManager):
        """Test get_model_info returns correct dictionary."""
        info = manager.get_model_info()
        assert "name" in info
        assert "version" in info
        assert "status" in info
        assert "device" in info

    def test_get_loaded_models_empty(self, manager: ModelManager):
        """Test get_loaded_models returns empty dict initially."""
        loaded = manager.get_loaded_models()
        assert isinstance(loaded, dict)
        assert len(loaded) == 0

    @pytest.mark.asyncio
    async def test_get_loaded_models_after_load(self, manager: ModelManager):
        """Test get_loaded_models returns loaded model."""
        await manager.load_model()
        loaded = manager.get_loaded_models()
        assert len(loaded) == 1
        assert "test_model" in loaded
