"""Tests for ModelRegistry."""

import pytest

from app.models.factory import YOLOModel
from app.models.registry import ModelRegistry


class TestModelRegistry:
    """Tests for ModelRegistry class."""

    @pytest.fixture
    def registry(self):
        """Create a fresh ModelRegistry."""
        return ModelRegistry()

    @pytest.fixture
    def sample_model(self):
        """Create a sample YOLO model."""
        return YOLOModel(name="test-yolo", version="1.0.0")

    def test_register_model(self, registry: ModelRegistry, sample_model):
        """Test registering a model."""
        registry.register_model(sample_model)
        assert registry.has_model("test-yolo")
        assert registry.count == 1

    def test_register_duplicate_raises(self, registry: ModelRegistry, sample_model):
        """Test registering duplicate model raises ValueError."""
        registry.register_model(sample_model)
        with pytest.raises(ValueError, match="already registered"):
            registry.register_model(sample_model)

    def test_unregister_model(self, registry: ModelRegistry, sample_model):
        """Test unregistering a model."""
        registry.register_model(sample_model)
        removed = registry.unregister_model("test-yolo")
        assert removed is sample_model
        assert not registry.has_model("test-yolo")
        assert registry.count == 0

    def test_unregister_nonexistent_raises(self, registry: ModelRegistry):
        """Test unregistering nonexistent model raises KeyError."""
        with pytest.raises(KeyError, match="not found"):
            registry.unregister_model("nonexistent")

    def test_get_model(self, registry: ModelRegistry, sample_model):
        """Test getting a model by name."""
        registry.register_model(sample_model)
        retrieved = registry.get_model("test-yolo")
        assert retrieved is sample_model

    def test_get_model_not_found_raises(self, registry: ModelRegistry):
        """Test getting nonexistent model raises KeyError."""
        with pytest.raises(KeyError, match="not found"):
            registry.get_model("nonexistent")

    def test_has_model(self, registry: ModelRegistry, sample_model):
        """Test has_model check."""
        assert not registry.has_model("test-yolo")
        registry.register_model(sample_model)
        assert registry.has_model("test-yolo")

    def test_list_models(self, registry: ModelRegistry, sample_model):
        """Test listing all models."""
        registry.register_model(sample_model)
        models = registry.list_models()
        assert "test-yolo" in models
        assert models["test-yolo"]["name"] == "test-yolo"
        assert models["test-yolo"]["model_type"] == "yolo"

    def test_list_model_names(self, registry: ModelRegistry):
        """Test listing model names returns sorted list."""
        m1 = YOLOModel(name="zebra-model")
        m2 = YOLOModel(name="alpha-model")
        registry.register_model(m1)
        registry.register_model(m2)
        names = registry.list_model_names()
        assert names == ["alpha-model", "zebra-model"]

    def test_set_default(self, registry: ModelRegistry, sample_model):
        """Test setting default model."""
        registry.register_model(sample_model)
        registry.set_default("test-yolo")
        assert registry.default_model_name == "test-yolo"
        assert registry.default_model() is sample_model

    def test_set_default_not_found_raises(self, registry: ModelRegistry):
        """Test setting nonexistent model as default raises KeyError."""
        with pytest.raises(KeyError, match="not found"):
            registry.set_default("nonexistent")

    def test_default_model_none_initially(self, registry: ModelRegistry):
        """Test default model is None initially."""
        assert registry.default_model() is None
        assert registry.default_model_name is None

    def test_unregister_clears_default(self, registry: ModelRegistry, sample_model):
        """Test unregistering default model clears default."""
        registry.register_model(sample_model)
        registry.set_default("test-yolo")
        registry.unregister_model("test-yolo")
        assert registry.default_model_name is None
        assert registry.default_model() is None

    def test_count(self, registry: ModelRegistry):
        """Test count tracks registered models."""
        assert registry.count == 0
        registry.register_model(YOLOModel(name="m1"))
        assert registry.count == 1
        registry.register_model(YOLOModel(name="m2"))
        assert registry.count == 2
        registry.unregister_model("m1")
        assert registry.count == 1
