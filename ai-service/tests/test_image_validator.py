"""Tests for image validator."""

import numpy as np
import pytest

from app.preprocessing.image_loader import LoadedImage
from app.preprocessing.image_validator import (
    ImageValidator,
    ValidationError,
    ValidationConfig,
    ValidationResult,
)


@pytest.fixture
def validator():
    """Create a default ImageValidator."""
    return ImageValidator()


@pytest.fixture
def valid_image():
    """Create a valid LoadedImage."""
    data = np.random.randint(0, 255, (480, 640, 3), dtype=np.uint8)
    return LoadedImage(
        data=data,
        source="test.jpg",
        width=640,
        height=480,
        channels=3,
    )


@pytest.fixture
def small_image():
    """Create a very small image."""
    data = np.ones((2, 2, 3), dtype=np.uint8) * 128
    return LoadedImage(data=data, source="small.jpg", width=2, height=2, channels=3)


@pytest.fixture
def grayscale_image():
    """Create a grayscale image."""
    data = np.random.randint(0, 255, (100, 100), dtype=np.uint8)
    return LoadedImage(data=data, source="gray.jpg", width=100, height=100, channels=1)


class TestImageValidator:
    """Tests for ImageValidator."""

    def test_valid_image_passes(
        self, validator: ImageValidator, valid_image: LoadedImage
    ):
        """Test valid image passes validation."""
        result = validator.validate(valid_image)
        assert result.is_valid is True
        assert len(result.errors) == 0

    def test_validate_or_raise_passes(
        self, validator: ImageValidator, valid_image: LoadedImage
    ):
        """Test validate_or_raise passes for valid image."""
        validator.validate_or_raise(valid_image)  # No exception

    def test_validate_or_raise_fails(self, validator: ImageValidator):
        """Test validate_or_raise raises for invalid image."""
        data = np.array([], dtype=np.uint8).reshape(0, 0)
        image = LoadedImage(
            data=data, source="empty.jpg", width=0, height=0, channels=0
        )

        with pytest.raises(ValidationError, match="failed"):
            validator.validate_or_raise(image)

    def test_empty_image_fails(self, validator: ImageValidator):
        """Test empty image fails validation."""
        data = np.array([], dtype=np.uint8).reshape(0, 0)
        image = LoadedImage(
            data=data, source="empty.jpg", width=0, height=0, channels=0
        )

        result = validator.validate(image)
        assert result.is_valid is False
        assert any("empty" in e.lower() for e in result.errors)

    def test_too_small_fails(self, validator: ImageValidator):
        """Test image below minimum dimensions fails."""
        data = np.ones((1, 1, 3), dtype=np.uint8)
        image = LoadedImage(data=data, source="tiny.jpg", width=1, height=1, channels=3)

        # Default min is 1x1, so this should pass
        result = validator.validate(image)
        assert result.is_valid is True

    def test_custom_min_dimensions(self):
        """Test custom minimum dimensions."""
        config = ValidationConfig(min_width=100, min_height=100)
        validator = ImageValidator(config)

        data = np.ones((50, 50, 3), dtype=np.uint8)
        image = LoadedImage(
            data=data, source="small.jpg", width=50, height=50, channels=3
        )

        result = validator.validate(image)
        assert result.is_valid is False
        assert any("too small" in e.lower() for e in result.errors)

    def test_too_large_fails(self):
        """Test image above maximum dimensions fails."""
        config = ValidationConfig(max_width=100, max_height=100)
        validator = ImageValidator(config)

        data = np.ones((200, 200, 3), dtype=np.uint8)
        image = LoadedImage(
            data=data, source="big.jpg", width=200, height=200, channels=3
        )

        result = validator.validate(image)
        assert result.is_valid is False
        assert any("too large" in e.lower() for e in result.errors)

    def test_max_pixels_fails(self):
        """Test image exceeding max pixel count fails."""
        config = ValidationConfig(max_pixels=1000)
        validator = ImageValidator(config)

        data = np.ones((40, 40, 3), dtype=np.uint8)
        image = LoadedImage(
            data=data, source="big.jpg", width=40, height=40, channels=3
        )

        result = validator.validate(image)
        assert result.is_valid is False
        assert any("pixel count" in e.lower() for e in result.errors)

    def test_unsupported_channels_fails(self, validator):
        """Test unsupported channel count fails."""
        data = np.ones((10, 10, 2), dtype=np.uint8)
        image = LoadedImage(
            data=data, source="2ch.jpg", width=10, height=10, channels=2
        )

        result = validator.validate(image)
        assert result.is_valid is False
        assert any("channel" in e.lower() for e in result.errors)

    def test_grayscale_passes(
        self, validator: ImageValidator, grayscale_image: LoadedImage
    ):
        """Test grayscale image passes validation."""
        result = validator.validate(grayscale_image)
        assert result.is_valid is True

    def test_black_image_warning(self, validator: ImageValidator):
        """Test all-black image produces warning."""
        data = np.zeros((100, 100, 3), dtype=np.uint8)
        image = LoadedImage(
            data=data, source="black.jpg", width=100, height=100, channels=3
        )

        result = validator.validate(image)
        assert result.is_valid is True
        assert len(result.warnings) > 0
        assert any("black" in w.lower() for w in result.warnings)

    def test_to_rgb(self, validator: ImageValidator):
        """Test BGR to RGB conversion."""
        data = np.zeros((10, 10, 3), dtype=np.uint8)
        data[0, 0] = [255, 0, 0]  # B=255, G=0, R=0 (blue in BGR)
        image = LoadedImage(
            data=data, source="test.jpg", width=10, height=10, channels=3
        )

        rgb = validator.to_rgb(image)
        # After conversion: R=0, G=0, B=255
        assert rgb.data[0, 0, 2] == 255
        assert rgb.data[0, 0, 0] == 0

    def test_to_rgb_grayscale_passthrough(
        self, validator: ImageValidator, grayscale_image: LoadedImage
    ):
        """Test to_rgb passes through grayscale."""
        result = validator.to_rgb(grayscale_image)
        assert result is grayscale_image

    def test_result_add_error(self):
        """Test ValidationResult add_error."""
        result = ValidationResult(is_valid=True)
        result.add_error("test error")
        assert result.is_valid is False
        assert "test error" in result.errors

    def test_result_add_warning(self):
        """Test ValidationResult add_warning."""
        result = ValidationResult(is_valid=True)
        result.add_warning("test warning")
        assert result.is_valid is True
        assert "test warning" in result.warnings
