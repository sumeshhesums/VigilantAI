"""Shared test fixtures for AI service tests."""

from pathlib import Path

import cv2
import numpy as np
import pytest


@pytest.fixture
def sample_image_bytes():
    """Generate a valid JPEG image as bytes."""
    img = np.random.randint(0, 255, (480, 640, 3), dtype=np.uint8)
    _, encoded = cv2.imencode(".jpg", img)
    return encoded.tobytes()


@pytest.fixture
def sample_png_bytes():
    """Generate a valid PNG image as bytes."""
    img = np.random.randint(0, 255, (480, 640, 3), dtype=np.uint8)
    _, encoded = cv2.imencode(".png", img)
    return encoded.tobytes()


@pytest.fixture
def sample_image_array():
    """Generate a sample image as NumPy array (HWC, BGR)."""
    return np.random.randint(0, 255, (480, 640, 3), dtype=np.uint8)


@pytest.fixture
def empty_image_bytes():
    """Return empty bytes."""
    return b""


@pytest.fixture
def corrupt_image_bytes():
    """Return corrupt image data."""
    return b"not a valid image"


@pytest.fixture
def assets_dir():
    """Return the test assets directory."""
    return Path(__file__).parent / "assets"
