"""Tests for image normalizer."""

import numpy as np
import pytest

from app.preprocessing.image_normalizer import (
    IMAGENET_MEAN,
    IMAGENET_STD,
    ImageNormalizer,
    NormalizationConfig,
    NormalizationMode,
    NormalizedImage,
)


@pytest.fixture
def normalizer_minmax():
    """Create a min-max normalizer."""
    return ImageNormalizer(NormalizationConfig(mode=NormalizationMode.MIN_MAX))


@pytest.fixture
def normalizer_imagenet():
    """Create an ImageNet normalizer."""
    return ImageNormalizer(NormalizationConfig(mode=NormalizationMode.IMAGENET))


@pytest.fixture
def sample_uint8():
    """Create a sample uint8 image."""
    return np.random.randint(0, 255, (640, 640, 3), dtype=np.uint8)


class TestImageNormalizer:
    """Tests for ImageNormalizer."""

    def test_min_max_output_dtype(self, normalizer_minmax, sample_uint8):
        """Test min-max produces float32."""
        result = normalizer_minmax.normalize(sample_uint8)
        assert isinstance(result, NormalizedImage)
        assert result.dtype == np.float32

    def test_min_max_range(self, normalizer_minmax, sample_uint8):
        """Test min-max scales to [0, 1]."""
        result = normalizer_minmax.normalize(sample_uint8)
        assert result.data.min() >= 0.0
        assert result.data.max() <= 1.0

    def test_min_max_zero_image(self, normalizer_minmax):
        """Test min-max on zero image."""
        img = np.zeros((10, 10, 3), dtype=np.uint8)
        result = normalizer_minmax.normalize(img)
        assert result.data.max() == 0.0

    def test_min_max_full_range(self, normalizer_minmax):
        """Test min-max on full-range image."""
        img = np.zeros((10, 10, 3), dtype=np.uint8)
        img[0, 0] = 255
        result = normalizer_minmax.normalize(img)
        assert result.data[0, 0, 0] == 1.0
        assert result.data[1, 1, 1] == 0.0

    def test_imagenet_output_dtype(self, normalizer_imagenet, sample_uint8):
        """Test ImageNet produces float32."""
        result = normalizer_imagenet.normalize(sample_uint8)
        assert result.dtype == np.float32

    def test_imagenet_shape_preserved(self, normalizer_imagenet, sample_uint8):
        """Test ImageNet preserves shape."""
        result = normalizer_imagenet.normalize(sample_uint8)
        assert result.shape == (640, 640, 3)

    def test_imagenet_uses_mean_std(self):
        """Test ImageNet normalization applies mean/std."""
        normalizer = ImageNormalizer(
            NormalizationConfig(mode=NormalizationMode.IMAGENET)
        )
        # Create uniform 128 pixel image
        img = np.full((10, 10, 3), 128, dtype=np.uint8)
        result = normalizer.normalize(img)

        # Manual computation
        expected = (128.0 / 255.0 - IMAGENET_MEAN) / IMAGENET_STD
        np.testing.assert_allclose(result.data[0, 0], expected, atol=1e-6)

    def test_grayscale_min_max(self, normalizer_minmax):
        """Test min-max on grayscale image."""
        img = np.random.randint(0, 255, (100, 100), dtype=np.uint8)
        result = normalizer_minmax.normalize(img)
        assert result.data.min() >= 0.0
        assert result.data.max() <= 1.0

    def test_normalized_image_properties(self, normalizer_minmax, sample_uint8):
        """Test NormalizedImage properties."""
        result = normalizer_minmax.normalize(sample_uint8)
        assert result.mode == NormalizationMode.MIN_MAX
        assert result.original_dtype == np.uint8
        assert isinstance(result.original_range, tuple)
        assert len(result.original_range) == 2

    def test_custom_normalization(self):
        """Test custom normalization mode."""
        config = NormalizationConfig(
            mode=NormalizationMode.CUSTOM,
            custom_mean=[0.5, 0.5, 0.5],
            custom_std=[0.5, 0.5, 0.5],
        )
        normalizer = ImageNormalizer(config)
        img = np.full((10, 10, 3), 128, dtype=np.uint8)
        result = normalizer.normalize(img)

        # (128/255 - 0.5) / 0.5
        expected = (128.0 / 255.0 - 0.5) / 0.5
        np.testing.assert_allclose(result.data[0, 0, 0], expected, atol=1e-6)

    def test_custom_requires_mean_std(self):
        """Test custom mode requires mean and std."""
        with pytest.raises(ValueError, match="requires"):
            NormalizationConfig(mode=NormalizationMode.CUSTOM)

    def test_custom_mismatched_lengths(self):
        """Test custom mode with mismatched mean/std lengths."""
        with pytest.raises(ValueError, match="same length"):
            NormalizationConfig(
                mode=NormalizationMode.CUSTOM,
                custom_mean=[0.5, 0.5, 0.5],
                custom_std=[0.5, 0.5],
            )

    def test_config_property(self, normalizer_minmax):
        """Test config property accessor."""
        assert normalizer_minmax.config.mode == NormalizationMode.MIN_MAX
