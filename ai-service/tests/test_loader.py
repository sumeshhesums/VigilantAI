"""Tests for ModelLoader."""

import pytest

from app.models.factory import RTDETRModel
from app.models.loader import ModelLoader
from app.models.registry import ModelRegistry


class TestModelLoader:
    """Tests for ModelLoader class."""

    @pytest.fixture
    def registry(self):
        """Create a registry with two models."""
        reg = ModelRegistry()
        reg.register_model(RTDETRModel(name="rtdetr-a"))
        reg.register_model(RTDETRModel(name="rtdetr-b"))
        return reg

    @pytest.fixture
    def loader(self, registry):
        """Create a ModelLoader with the registry."""
        return ModelLoader(registry)

    @pytest.mark.asyncio
    async def test_load_model(self, loader: ModelLoader):
        """Test loading a single model."""
        result = await loader.load_model("rtdetr-a")
        assert result["state"] == "loaded"
        assert result["name"] == "rtdetr-a"

    @pytest.mark.asyncio
    async def test_load_model_not_found(self, loader: ModelLoader):
        """Test loading nonexistent model raises KeyError."""
        with pytest.raises(KeyError, match="not found"):
            await loader.load_model("nonexistent")

    @pytest.mark.asyncio
    async def test_unload_model(self, loader: ModelLoader):
        """Test unloading a model."""
        await loader.load_model("rtdetr-a")
        await loader.unload_model("rtdetr-a")
        status = loader.get_model_status("rtdetr-a")
        assert status["state"] == "not_loaded"

    @pytest.mark.asyncio
    async def test_reload_model(self, loader: ModelLoader):
        """Test reloading a model."""
        await loader.load_model("rtdetr-a")
        result = await loader.reload_model("rtdetr-a")
        assert result["state"] == "loaded"

    @pytest.mark.asyncio
    async def test_load_all(self, loader: ModelLoader):
        """Test loading all models."""
        results = await loader.load_all()
        assert len(results) == 2
        assert "rtdetr-a" in results
        assert "rtdetr-b" in results
        assert results["rtdetr-a"]["state"] == "loaded"
        assert results["rtdetr-b"]["state"] == "loaded"

    @pytest.mark.asyncio
    async def test_unload_all(self, loader: ModelLoader):
        """Test unloading all models."""
        await loader.load_all()
        await loader.unload_all()
        for name in ["rtdetr-a", "rtdetr-b"]:
            status = loader.get_model_status(name)
            assert status["state"] == "not_loaded"

    @pytest.mark.asyncio
    async def test_warmup_model(self, loader: ModelLoader):
        """Test warming up a loaded model."""
        await loader.load_model("rtdetr-a")
        result = await loader.warmup_model("rtdetr-a")
        assert result["state"] == "loaded"

    @pytest.mark.asyncio
    async def test_warmup_unloaded_raises(self, loader: ModelLoader):
        """Test warming up unloaded model raises RuntimeError."""
        with pytest.raises(RuntimeError, match="must be loaded"):
            await loader.warmup_model("rtdetr-a")

    def test_get_model_status(self, loader: ModelLoader):
        """Test getting model status."""
        status = loader.get_model_status("rtdetr-a")
        assert "name" in status
        assert status["name"] == "rtdetr-a"
        assert status["state"] == "not_loaded"

    def test_get_all_statuses(self, loader: ModelLoader):
        """Test getting all model statuses."""
        statuses = loader.get_all_statuses()
        assert len(statuses) == 2
        assert "rtdetr-a" in statuses
        assert "rtdetr-b" in statuses
