"""Image loader supporting JPEG, PNG, and BMP formats."""

from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Union

import cv2
import numpy as np

from app.logging import get_logger

logger = get_logger(__name__)

SUPPORTED_EXTENSIONS = {".jpg", ".jpeg", ".png", ".bmp"}


@dataclass
class LoadedImage:
    """Container for a loaded image and its metadata."""

    data: np.ndarray
    """Image as NumPy array in HWC (height, width, channels) format, BGR."""

    source: str
    """Original source path or identifier."""

    width: int
    """Image width in pixels."""

    height: int
    """Image height in pixels."""

    channels: int
    """Number of color channels."""

    @property
    def shape(self) -> tuple[int, int, int]:
        """Return (height, width, channels) tuple."""
        return (self.height, self.width, self.channels)

    @property
    def is_rgb(self) -> bool:
        """Check if image has 3 channels (RGB/BGR)."""
        return self.channels == 3

    @property
    def is_grayscale(self) -> bool:
        """Check if image is single-channel."""
        return self.channels == 1


class ImageLoader:
    """Loads images from files or bytes into NumPy arrays.

    Supports JPEG, PNG, and BMP formats.
    Images are loaded as BGR (OpenCV default).
    """

    def __init__(
        self,
        *,
        use_bgr: bool = True,
    ) -> None:
        """Initialize the loader.

        Args:
            use_bgr: If True, load as BGR (OpenCV default).
                     If False, convert to RGB on load.
        """
        self._use_bgr = use_bgr

    def load_from_file(self, path: Union[str, Path]) -> LoadedImage:
        """Load an image from a file path.

        Args:
            path: Path to the image file.

        Returns:
            LoadedImage with the image data and metadata.

        Raises:
            FileNotFoundError: If file does not exist.
            ValueError: If file extension is not supported.
            RuntimeError: If image cannot be decoded.
        """
        path = Path(path)

        if not path.exists():
            raise FileNotFoundError(f"Image file not found: {path}")

        if path.suffix.lower() not in SUPPORTED_EXTENSIONS:
            raise ValueError(
                f"Unsupported image format: {path.suffix}. "
                f"Supported: {sorted(SUPPORTED_EXTENSIONS)}"
            )

        return self._decode_image(
            cv2.imread(str(path), cv2.IMREAD_COLOR),
            source=str(path),
        )

    def load_from_bytes(
        self,
        data: bytes,
        source: str = "<bytes>",
    ) -> LoadedImage:
        """Load an image from raw bytes.

        Args:
            data: Raw image bytes.
            source: Identifier for error messages.

        Returns:
            LoadedImage with the image data and metadata.

        Raises:
            ValueError: If data is empty.
            RuntimeError: If image cannot be decoded.
        """
        if not data:
            raise ValueError("Image data is empty")

        nparr = np.frombuffer(data, np.uint8)
        img = cv2.imdecode(nparr, cv2.IMREAD_COLOR)

        return self._decode_image(img, source=source)

    def _decode_image(
        self,
        img: np.ndarray | None,
        source: str,
    ) -> LoadedImage:
        """Validate decoded image and wrap in LoadedImage.

        Args:
            img: Decoded NumPy array or None.
            source: Source identifier.

        Returns:
            LoadedImage instance.

        Raises:
            RuntimeError: If image decode failed.
        """
        if img is None:
            raise RuntimeError(f"Failed to decode image from: {source}")

        if img.size == 0:
            raise RuntimeError(f"Decoded image is empty: {source}")

        h, w = img.shape[:2]
        c = img.shape[2] if len(img.shape) > 2 else 1

        if not self._use_bgr:
            if c == 3:
                img = cv2.cvtColor(img, cv2.COLOR_BGR2RGB)

        logger.debug("Loaded image %s: %dx%dx%d", source, w, h, c)

        return LoadedImage(
            data=img,
            source=source,
            width=w,
            height=h,
            channels=c,
        )
