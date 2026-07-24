"""Tests for image resizer."""

import numpy as np
import pytest

from app.preprocessing.image_loader import LoadedImage
from app.preprocessing.image_resizer import (
    ImageResizer,
    ResizeConfig,
    ResizeMode,
    ResizedImage,
)


@pytest.fixture
def resizer_letterbox():
    """Create a letterbox resizer."""
    return ImageResizer(
        ResizeConfig(target_width=640, target_height=640, mode=ResizeMode.LETTERBOX)
    )


@pytest.fixture
def resizer_stretch():
    """Create a stretch resizer."""
    return ImageResizer(
        ResizeConfig(target_width=640, target_height=640, mode=ResizeMode.STRETCH)
    )


@pytest.fixture
def resizer_aspect():
    """Create an aspect-ratio resizer."""
    return ImageResizer(
        ResizeConfig(target_width=640, target_height=640, mode=ResizeMode.ASPECT_RATIO)
    )


@pytest.fixture
def landscape_image():
    """Create a landscape image 1280x720."""
    data = np.random.randint(0, 255, (720, 1280, 3), dtype=np.uint8)
    return LoadedImage(
        data=data, source="landscape.jpg", width=1280, height=720, channels=3
    )


@pytest.fixture
def portrait_image():
    """Create a portrait image 360x640."""
    data = np.random.randint(0, 255, (640, 360, 3), dtype=np.uint8)
    return LoadedImage(
        data=data, source="portrait.jpg", width=360, height=640, channels=3
    )


@pytest.fixture
def square_image():
    """Create a square image 400x400."""
    data = np.random.randint(0, 255, (400, 400, 3), dtype=np.uint8)
    return LoadedImage(
        data=data, source="square.jpg", width=400, height=400, channels=3
    )


class TestImageResizer:
    """Tests for ImageResizer."""

    def test_letterbox_output_size(self, resizer_letterbox, landscape_image):
        """Test letterbox produces exact target dimensions."""
        result = resizer_letterbox.resize(landscape_image)
        assert isinstance(result, ResizedImage)
        assert result.width == 640
        assert result.height == 640

    def test_letterbox_maintains_aspect_ratio(self, resizer_letterbox, landscape_image):
        """Test letterbox adds padding, not distortion."""
        result = resizer_letterbox.resize(landscape_image)
        assert result.pad_x > 0 or result.pad_y > 0
        assert result.scale < 1.0

    def test_letterbox_centered_padding(self, resizer_letterbox, landscape_image):
        """Test letterbox centers the image."""
        result = resizer_letterbox.resize(landscape_image)
        assert result.pad_y > result.pad_x

    def test_stretch_output_size(self, resizer_stretch, landscape_image):
        """Test stretch produces exact target dimensions."""
        result = resizer_stretch.resize(landscape_image)
        assert result.width == 640
        assert result.height == 640

    def test_stretch_no_padding(self, resizer_stretch, landscape_image):
        """Test stretch adds no padding."""
        result = resizer_stretch.resize(landscape_image)
        assert result.pad_x == 0
        assert result.pad_y == 0

    def test_aspect_ratio_output_size(self, resizer_aspect, landscape_image):
        """Test aspect-ratio produces target dimensions."""
        result = resizer_aspect.resize(landscape_image)
        assert result.width == 640
        assert result.height == 640

    def test_aspect_ratio_scales_uniformly(self, resizer_aspect, landscape_image):
        """Test aspect-ratio scales uniformly."""
        result = resizer_aspect.resize(landscape_image)
        assert result.scale < 1.0

    def test_square_image_letterbox(self, resizer_letterbox, square_image):
        """Test letterbox on square image has no padding."""
        result = resizer_letterbox.resize(square_image)
        assert result.pad_x == 0
        assert result.pad_y == 0

    def test_portrait_image_letterbox(self, resizer_letterbox, portrait_image):
        """Test letterbox on portrait image adds horizontal padding."""
        result = resizer_letterbox.resize(portrait_image)
        assert result.pad_x > result.pad_y

    def test_resize_config_validation(self):
        """Test invalid config raises ValueError."""
        with pytest.raises(ValueError, match="must be positive"):
            ResizeConfig(target_width=0, target_height=640)

    def test_resize_config_negative(self):
        """Test negative dimension raises ValueError."""
        with pytest.raises(ValueError, match="must be positive"):
            ResizeConfig(target_width=640, target_height=-1)

    def test_resized_image_properties(self, resizer_letterbox, landscape_image):
        """Test ResizedImage property accessors."""
        result = resizer_letterbox.resize(landscape_image)
        assert result.original_width == 1280
        assert result.original_height == 720
        assert result.target_width == 640
        assert result.target_height == 640
        assert result.channels == 3
        assert result.data.shape == (640, 640, 3)

    def test_custom_letterbox_color(self, landscape_image):
        """Test custom letterbox fill color."""
        config = ResizeConfig(
            target_width=640,
            target_height=640,
            mode=ResizeMode.LETTERBOX,
            letterbox_color=(0, 0, 0),
        )
        resizer = ImageResizer(config)
        result = resizer.resize(landscape_image)

        # Check padding area is black
        if result.pad_y > 0:
            top_row = result.data[0, :]
            assert np.all(top_row == 0)

    def test_small_to_large_resize(self):
        """Test resizing small image to large target."""
        data = np.ones((10, 10, 3), dtype=np.uint8) * 128
        image = LoadedImage(
            data=data, source="small.jpg", width=10, height=10, channels=3
        )

        resizer = ImageResizer(ResizeConfig(target_width=320, target_height=320))
        result = resizer.resize(image)
        assert result.width == 320
        assert result.height == 320
