"""Tests for image loader."""

from pathlib import Path

import cv2
import numpy as np
import pytest

from app.preprocessing.image_loader import (
    ImageLoader,
    LoadedImage,
    SUPPORTED_EXTENSIONS,
)


@pytest.fixture
def loader():
    """Create a default ImageLoader."""
    return ImageLoader()


@pytest.fixture
def sample_image_array():
    """Create a sample BGR image as NumPy array."""
    return np.random.randint(0, 255, (480, 640, 3), dtype=np.uint8)


@pytest.fixture
def sample_grayscale_array():
    """Create a sample grayscale image as NumPy array."""
    return np.random.randint(0, 255, (480, 640), dtype=np.uint8)


class TestImageLoader:
    """Tests for ImageLoader."""

    def test_load_from_file_jpeg(self, loader: ImageLoader, tmp_path: Path):
        """Test loading JPEG image."""
        img = np.random.randint(0, 255, (100, 100, 3), dtype=np.uint8)
        path = tmp_path / "test.jpg"
        cv2.imwrite(str(path), img)

        result = loader.load_from_file(path)
        assert isinstance(result, LoadedImage)
        assert result.width == 100
        assert result.height == 100
        assert result.channels == 3

    def test_load_from_file_png(self, loader: ImageLoader, tmp_path: Path):
        """Test loading PNG image."""
        img = np.random.randint(0, 255, (80, 120, 3), dtype=np.uint8)
        path = tmp_path / "test.png"
        cv2.imwrite(str(path), img)

        result = loader.load_from_file(path)
        assert result.width == 120
        assert result.height == 80
        assert result.channels == 3

    def test_load_from_file_bmp(self, loader: ImageLoader, tmp_path: Path):
        """Test loading BMP image."""
        img = np.random.randint(0, 255, (60, 60, 3), dtype=np.uint8)
        path = tmp_path / "test.bmp"
        cv2.imwrite(str(path), img)

        result = loader.load_from_file(path)
        assert result.width == 60
        assert result.height == 60

    def test_load_from_file_not_found(self, loader: ImageLoader):
        """Test loading nonexistent file raises FileNotFoundError."""
        with pytest.raises(FileNotFoundError, match="not found"):
            loader.load_from_file("/nonexistent/image.jpg")

    def test_load_from_file_unsupported_format(
        self, loader: ImageLoader, tmp_path: Path
    ):
        """Test loading unsupported format raises ValueError."""
        path = tmp_path / "test.gif"
        path.write_bytes(b"fake gif content")

        with pytest.raises(ValueError, match="Unsupported image format"):
            loader.load_from_file(path)

    def test_load_from_bytes(self, loader: ImageLoader):
        """Test loading from raw bytes."""
        img = np.random.randint(0, 255, (50, 50, 3), dtype=np.uint8)
        _, encoded = cv2.imencode(".jpg", img)

        result = loader.load_from_bytes(encoded.tobytes())
        assert isinstance(result, LoadedImage)
        assert result.channels == 3

    def test_load_from_bytes_empty(self, loader: ImageLoader):
        """Test loading empty bytes raises ValueError."""
        with pytest.raises(ValueError, match="empty"):
            loader.load_from_bytes(b"")

    def test_load_from_bytes_corrupt(self, loader: ImageLoader):
        """Test loading corrupt bytes raises RuntimeError."""
        with pytest.raises(RuntimeError, match="Failed to decode"):
            loader.load_from_bytes(b"not an image at all")

    def test_loadedImage_properties(self, loader: ImageLoader):
        """Test LoadedImage properties."""
        img = np.random.randint(0, 255, (100, 200, 3), dtype=np.uint8)
        _, encoded = cv2.imencode(".jpg", img)
        result = loader.load_from_bytes(encoded.tobytes())

        assert result.shape == (100, 200, 3)
        assert result.is_rgb is True
        assert result.is_grayscale is False

    def test_loaded_grayscale(self):
        """Test LoadedImage loads grayscale as 3-channel (OpenCV default)."""
        loader = ImageLoader()
        img = np.random.randint(0, 255, (50, 50), dtype=np.uint8)
        _, encoded = cv2.imencode(".png", img)
        result = loader.load_from_bytes(encoded.tobytes())
        assert result.channels == 3
        assert result.width == 50
        assert result.height == 50

    def test_load_use_rgb_false(self, tmp_path: Path):
        """Test loading with use_bgr=False returns BGR still (via OpenCV)."""
        loader = ImageLoader(use_bgr=False)
        img = np.random.randint(0, 255, (50, 50, 3), dtype=np.uint8)
        path = tmp_path / "test.jpg"
        cv2.imwrite(str(path), img)

        result = loader.load_from_file(path)
        # OpenCV loads as BGR, then we convert to RGB when use_bgr=False
        assert result.channels == 3

    def test_supported_extensions_defined(self):
        """Test SUPPORTED_EXTENSIONS is populated."""
        assert ".jpg" in SUPPORTED_EXTENSIONS
        assert ".jpeg" in SUPPORTED_EXTENSIONS
        assert ".png" in SUPPORTED_EXTENSIONS
        assert ".bmp" in SUPPORTED_EXTENSIONS
