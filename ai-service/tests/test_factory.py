"""Tests for model factory implementations."""

import pytest

from app.models.base import ModelState
from app.models.factory import YOLOModel, RTDETRModel, GroundingDINOModel


class TestYOLOModel:
    """Tests for YOLOModel."""

    @pytest.mark.asyncio
    async def test_load_sets_metadata(self):
        """Test loading YOLO model sets correct metadata."""
        model = YOLOModel(name="yolov8n", version="8.0.0", device="cpu")
        await model.load()
        assert model.is_loaded
        assert model.metadata.input_shape == [1, 3, 640, 640]
        assert model.metadata.class_count == 80
        assert model.metadata.model_type == "yolo"
        assert model.metadata.extra["framework"] == "ultralytics"

    @pytest.mark.asyncio
    async def test_unload_clears_metadata(self):
        """Test unloading YOLO model clears metadata."""
        model = YOLOModel()
        await model.load()
        await model.unload()
        assert not model.is_loaded
        assert model.metadata.input_shape is None
        assert model.metadata.class_count is None

    @pytest.mark.asyncio
    async def test_default_name_and_version(self):
        """Test default name and version."""
        model = YOLOModel()
        assert model.name == "yolov8n"
        assert model.metadata.version == "8.0.0"

    def test_health_report(self):
        """Test health report structure."""
        model = YOLOModel()
        health = model.health()
        assert "name" in health
        assert "state" in health
        assert "loaded" in health
        assert health["loaded"] is False


class TestRTDETRModel:
    """Tests for RTDETRModel."""

    @pytest.mark.asyncio
    async def test_load_sets_metadata(self):
        """Test loading RTDETR model sets correct metadata."""
        model = RTDETRModel(name="rtdetr-l")
        await model.load()
        assert model.is_loaded
        assert model.metadata.input_shape == [1, 3, 640, 640]
        assert model.metadata.model_type == "rtdetr"

    @pytest.mark.asyncio
    async def test_unload(self):
        """Test unloading RTDETR model."""
        model = RTDETRModel()
        await model.load()
        await model.unload()
        assert not model.is_loaded

    def test_default_name(self):
        """Test default name."""
        model = RTDETRModel()
        assert model.name == "rtdetr-l"


class TestGroundingDINOModel:
    """Tests for GroundingDINOModel."""

    @pytest.mark.asyncio
    async def test_load_sets_metadata(self):
        """Test loading Grounding DINO model sets correct metadata."""
        model = GroundingDINOModel()
        await model.load()
        assert model.is_loaded
        assert model.metadata.input_shape == [1, 3, 800, 800]
        assert model.metadata.class_count is None  # Open-set
        assert model.metadata.model_type == "grounding_dino"
        assert model.metadata.extra["task"] == "open-set-detect"

    @pytest.mark.asyncio
    async def test_unload(self):
        """Test unloading Grounding DINO model."""
        model = GroundingDINOModel()
        await model.load()
        await model.unload()
        assert not model.is_loaded

    def test_default_name(self):
        """Test default name."""
        model = GroundingDINOModel()
        assert model.name == "grounding-dino-tiny"


class TestBaseModelLifecycle:
    """Tests for BaseModel lifecycle via concrete classes."""

    @pytest.mark.asyncio
    async def test_load_idempotent(self):
        """Test loading already loaded model returns immediately."""
        model = YOLOModel()
        m1 = await model.load()
        m2 = await model.load()
        assert m1 is m2

    @pytest.mark.asyncio
    async def test_warmup_after_load(self):
        """Test warmup works after loading."""
        model = YOLOModel()
        await model.load()
        result = await model.warmup()
        assert result.state == ModelState.LOADED

    @pytest.mark.asyncio
    async def test_warmup_before_load_raises(self):
        """Test warmup before load raises RuntimeError."""
        model = YOLOModel()
        with pytest.raises(RuntimeError, match="must be loaded"):
            await model.warmup()

    @pytest.mark.asyncio
    async def test_unload_when_not_loaded_is_safe(self):
        """Test unloading when not loaded is a no-op."""
        model = YOLOModel()
        await model.unload()
        assert not model.is_loaded
