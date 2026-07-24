"""Tests for the multi-model ModelManager."""

import pytest

from app.core.model_manager import ModelManager
from app.models.factory import RTDETRModel, GroundingDINOModel
from app.models.loader import ModelLoader
from app.models.registry import ModelRegistry


@pytest.fixture
def registry():
    """Create a registry with multiple models."""
    reg = ModelRegistry()
    reg.register_model(RTDETRModel(name="rtdetr-l"))
    reg.register_model(GroundingDINOModel(name="grounding-dino-tiny"))
    reg.set_default("rtdetr-l")
    return reg


@pytest.fixture
def manager(registry):
    """Create a ModelManager with the registry."""
    loader = ModelLoader(registry)
    return ModelManager(
        registry=registry,
        loader=loader,
        default_model_name="rtdetr-l",
    )


class TestModelManagerMultiModel:
    """Tests for multi-model ModelManager."""

    def test_initial_active_model(self, manager: ModelManager):
        """Test initial active model is set from default."""
        assert manager.active_model_name == "rtdetr-l"

    def test_active_model_property(self, manager: ModelManager):
        """Test active_model returns the model instance."""
        model = manager.active_model
        assert model is not None
        assert model.name == "rtdetr-l"

    def test_set_active_model(self, manager: ModelManager):
        """Test setting active model."""
        manager.set_active_model("grounding-dino-tiny")
        assert manager.active_model_name == "grounding-dino-tiny"
        assert manager.active_model.name == "grounding-dino-tiny"

    def test_set_active_model_not_found(self, manager: ModelManager):
        """Test setting nonexistent model as active raises KeyError."""
        with pytest.raises(KeyError, match="not found"):
            manager.set_active_model("nonexistent")

    @pytest.mark.asyncio
    async def test_switch_model(self, manager: ModelManager):
        """Test switching to another model loads it if needed."""
        result = await manager.switch_model("grounding-dino-tiny")
        assert manager.active_model_name == "grounding-dino-tiny"
        assert result["state"] == "loaded"

    @pytest.mark.asyncio
    async def test_switch_model_not_found(self, manager: ModelManager):
        """Test switching to nonexistent model raises KeyError."""
        with pytest.raises(KeyError, match="not found"):
            await manager.switch_model("nonexistent")

    @pytest.mark.asyncio
    async def test_load_by_name(self, manager: ModelManager):
        """Test loading a model by name."""
        result = await manager.load_by_name("rtdetr-l")
        assert result["state"] == "loaded"

    @pytest.mark.asyncio
    async def test_unload_by_name(self, manager: ModelManager):
        """Test unloading a model by name."""
        await manager.load_by_name("rtdetr-l")
        await manager.unload_by_name("rtdetr-l")
        status = manager.get_status("rtdetr-l")
        assert status["state"] == "not_loaded"

    @pytest.mark.asyncio
    async def test_unload_active_clears_active(self, manager: ModelManager):
        """Test unloading the active model clears active."""
        await manager.load_by_name("rtdetr-l")
        await manager.unload_by_name("rtdetr-l")
        assert manager.active_model_name is None

    @pytest.mark.asyncio
    async def test_reload_by_name(self, manager: ModelManager):
        """Test reloading a model by name."""
        await manager.load_by_name("rtdetr-l")
        result = await manager.reload_by_name("rtdetr-l")
        assert result["state"] == "loaded"

    def test_get_status(self, manager: ModelManager):
        """Test getting model status."""
        status = manager.get_status("rtdetr-l")
        assert status["name"] == "rtdetr-l"
        assert status["state"] == "not_loaded"

    def test_get_all_statuses(self, manager: ModelManager):
        """Test getting all model statuses."""
        statuses = manager.get_all_statuses()
        assert len(statuses) == 2
        assert "rtdetr-l" in statuses
        assert "grounding-dino-tiny" in statuses

    def test_list_models(self, manager: ModelManager):
        """Test listing all models."""
        models = manager.list_models()
        assert len(models) == 2

    def test_get_model_info_legacy(self, manager: ModelManager):
        """Test legacy get_model_info returns active model info."""
        info = manager.get_model_info()
        assert info["name"] == "rtdetr-l"

    def test_get_model_info_no_active(self):
        """Test get_model_info with no active model returns defaults."""
        reg = ModelRegistry()
        loader = ModelLoader(reg)
        mgr = ModelManager(registry=reg, loader=loader)
        info = mgr.get_model_info()
        assert info["name"] == "unknown"

    def test_is_loaded_false_initially(self, manager: ModelManager):
        """Test is_loaded is False when model not loaded."""
        assert not manager.is_loaded

    @pytest.mark.asyncio
    async def test_is_loaded_true_after_load(self, manager: ModelManager):
        """Test is_loaded is True when active model is loaded."""
        await manager.load_by_name("rtdetr-l")
        assert manager.is_loaded

    def test_metadata_legacy(self, manager: ModelManager):
        """Test legacy metadata property."""
        meta = manager.metadata
        assert meta.name == "rtdetr-l"

    def test_registry_property(self, manager: ModelManager):
        """Test registry property access."""
        assert manager.registry is not None
        assert manager.registry.count == 2

    def test_loader_property(self, manager: ModelManager):
        """Test loader property access."""
        assert manager.loader is not None
