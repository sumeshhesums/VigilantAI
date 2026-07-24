"""Tests for preprocessing pipeline."""

from pathlib import Path

import cv2
import numpy as np
import pytest

from app.preprocessing.image_validator import ValidationError
from app.preprocessing.pipeline import (
    PipelineConfig,
    PreprocessedImage,
    PreprocessingPipeline,
)
from app.preprocessing.image_normalizer import NormalizationConfig, NormalizationMode
from app.preprocessing.image_resizer import ResizeConfig, ResizeMode


@pytest.fixture
def pipeline():
    """Create a default preprocessing pipeline."""
    return PreprocessingPipeline()


@pytest.fixture
def pipeline_640():
    """Create a pipeline targeting 640x640."""
    return PreprocessingPipeline(
        PipelineConfig(
            resize=ResizeConfig(target_width=640, target_height=640),
        )
    )


@pytest.fixture
def landscape_jpeg(tmp_path: Path):
    """Create a temporary JPEG image file."""
    img = np.random.randint(0, 255, (360, 640, 3), dtype=np.uint8)
    path = tmp_path / "test.jpg"
    cv2.imwrite(str(path), img)
    return path


@pytest.fixture
def portrait_png(tmp_path: Path):
    """Create a temporary PNG image file."""
    img = np.random.randint(0, 255, (640, 360, 3), dtype=np.uint8)
    path = tmp_path / "test.png"
    cv2.imwrite(str(path), img)
    return path


@pytest.fixture
def small_jpeg(tmp_path: Path):
    """Create a very small JPEG image."""
    img = np.ones((10, 10, 3), dtype=np.uint8) * 128
    path = tmp_path / "small.jpg"
    cv2.imwrite(str(path), img)
    return path


class TestPreprocessingPipeline:
    """Tests for PreprocessingPipeline."""

    def test_process_file_returns_preprocessed(self, pipeline, landscape_jpeg):
        """Test process_file returns PreprocessedImage."""
        result = pipeline.process_file(landscape_jpeg)
        assert isinstance(result, PreprocessedImage)
        assert result.data.dtype == np.float32

    def test_process_file_correct_shape(self, pipeline_640, landscape_jpeg):
        """Test process_file returns correct output shape."""
        result = pipeline_640.process_file(landscape_jpeg)
        assert result.data.shape == (640, 640, 3)

    def test_process_file_metadata(self, pipeline, landscape_jpeg):
        """Test process_file preserves metadata."""
        result = pipeline.process_file(landscape_jpeg)
        assert result.original_width == 640
        assert result.original_height == 360
        assert result.processing_time_ms > 0
        assert result.source == str(landscape_jpeg)

    def test_process_file_not_found(self, pipeline):
        """Test process_file with nonexistent file."""
        with pytest.raises(FileNotFoundError):
            pipeline.process_file("/nonexistent/image.jpg")

    def test_process_file_unsupported_format(self, pipeline, tmp_path):
        """Test process_file with unsupported format."""
        path = tmp_path / "test.gif"
        path.write_bytes(b"fake")

        with pytest.raises((ValueError, RuntimeError)):
            pipeline.process_file(path)

    def test_process_bytes(self, pipeline):
        """Test process_bytes with encoded image."""
        img = np.random.randint(0, 255, (100, 100, 3), dtype=np.uint8)
        _, encoded = cv2.imencode(".jpg", img)

        result = pipeline.process_bytes(encoded.tobytes())
        assert isinstance(result, PreprocessedImage)
        assert result.data.dtype == np.float32

    def test_process_bytes_empty(self, pipeline):
        """Test process_bytes with empty data."""
        with pytest.raises(ValueError, match="empty"):
            pipeline.process_bytes(b"")

    def test_process_bytes_corrupt(self, pipeline):
        """Test process_bytes with corrupt data."""
        with pytest.raises(RuntimeError, match="Failed to decode"):
            pipeline.process_bytes(b"not an image")

    def test_process_image(self, pipeline):
        """Test process_image with pre-loaded image."""
        from app.preprocessing.image_loader import LoadedImage

        data = np.random.randint(0, 255, (200, 200, 3), dtype=np.uint8)
        image = LoadedImage(
            data=data, source="test.jpg", width=200, height=200, channels=3
        )

        result = pipeline.process_image(image)
        assert isinstance(result, PreprocessedImage)

    def test_validation_failure(self, tmp_path):
        """Test pipeline fails on invalid image."""
        # Create image that exceeds max dimensions
        img = np.ones((10, 10, 3), dtype=np.uint8)
        path = tmp_path / "test.jpg"
        cv2.imwrite(str(path), img)

        from app.preprocessing.image_validator import ValidationConfig

        narrow_config = PipelineConfig(
            validation=ValidationConfig(max_width=5, max_height=5),
            resize=ResizeConfig(target_width=640, target_height=640),
        )
        narrow_pipeline = PreprocessingPipeline(narrow_config)

        with pytest.raises(ValidationError):
            narrow_pipeline.process_file(path)

    def test_pipeline_config_default(self):
        """Test default PipelineConfig."""
        config = PipelineConfig()
        assert config.convert_to_rgb is True
        assert config.resize.target_width == 640
        assert config.resize.target_height == 640

    def test_pipeline_properties(self, pipeline):
        """Test pipeline property accessors."""
        assert pipeline.config is not None
        assert pipeline.loader is not None
        assert pipeline.validator is not None
        assert pipeline.resizer is not None
        assert pipeline.normalizer is not None

    def test_pipeline_portrait_image(self, pipeline_640, portrait_png):
        """Test pipeline with portrait image."""
        result = pipeline_640.process_file(portrait_png)
        assert result.data.shape == (640, 640, 3)
        assert result.original_width == 360
        assert result.original_height == 640

    def test_pipeline_small_image(self, pipeline_640, small_jpeg):
        """Test pipeline upscales small image."""
        result = pipeline_640.process_file(small_jpeg)
        assert result.data.shape == (640, 640, 3)

    def test_pipeline_imagenet_normalization(self, landscape_jpeg):
        """Test pipeline with ImageNet normalization."""
        config = PipelineConfig(
            resize=ResizeConfig(target_width=640, target_height=640),
            normalization=NormalizationConfig(mode=NormalizationMode.IMAGENET),
        )
        pipeline = PreprocessingPipeline(config)
        result = pipeline.process_file(landscape_jpeg)
        assert result.data.dtype == np.float32
        assert result.data.shape == (640, 640, 3)

    def test_pipeline_stretch_resize(self, landscape_jpeg):
        """Test pipeline with stretch resize mode."""
        config = PipelineConfig(
            resize=ResizeConfig(
                target_width=640, target_height=640, mode=ResizeMode.STRETCH
            ),
        )
        pipeline = PreprocessingPipeline(config)
        result = pipeline.process_file(landscape_jpeg)
        assert result.pad_x == 0
        assert result.pad_y == 0

    def test_preprocessed_image_shape(self, pipeline_640, landscape_jpeg):
        """Test PreprocessedImage shape accessor."""
        result = pipeline_640.process_file(landscape_jpeg)
        assert result.shape == (640, 640, 3)

    def test_preprocessed_image_dtype(self, pipeline_640, landscape_jpeg):
        """Test PreprocessedImage dtype accessor."""
        result = pipeline_640.process_file(landscape_jpeg)
        assert result.dtype == np.float32
