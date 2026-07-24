"""Image resizer supporting letterbox, stretch, and aspect-ratio modes."""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum

import cv2
import numpy as np

from app.logging import get_logger
from app.preprocessing.image_loader import LoadedImage

logger = get_logger(__name__)


class ResizeMode(str, Enum):
    """Resize strategy enumeration."""

    LETTERBOX = "letterbox"
    """Pad with letterbox to maintain aspect ratio, fill with gray."""

    STRETCH = "stretch"
    """Direct resize, may distort aspect ratio."""

    ASPECT_RATIO = "aspect_ratio"
    """Resize to fit within target, maintaining aspect ratio, may crop or pad."""


@dataclass
class ResizeConfig:
    """Configuration for image resizing."""

    target_width: int = 640
    target_height: int = 640
    mode: ResizeMode = ResizeMode.LETTERBOX
    letterbox_color: tuple[int, int, int] = (114, 114, 114)
    """Fill color for letterbox padding (BGR)."""

    def __post_init__(self) -> None:
        if self.target_width <= 0 or self.target_height <= 0:
            raise ValueError(
                f"Target dimensions must be positive: "
                f"{self.target_width}x{self.target_height}"
            )


@dataclass
class ResizedImage:
    """Container for a resized image with resize metadata."""

    data: np.ndarray
    """Resized image as NumPy array."""

    original_width: int
    """Original image width."""

    original_height: int
    """Original image height."""

    target_width: int
    """Target width."""

    target_height: int
    """Target height."""

    scale: float
    """Scale factor applied during resize."""

    pad_x: int
    """Horizontal padding added (letterbox)."""

    pad_y: int
    """Vertical padding added (letterbox)."""

    @property
    def width(self) -> int:
        """Actual output width."""
        return self.data.shape[1]

    @property
    def height(self) -> int:
        """Actual output height."""
        return self.data.shape[0]

    @property
    def channels(self) -> int:
        """Number of channels."""
        if len(self.data.shape) == 2:
            return 1
        return self.data.shape[2]


class ImageResizer:
    """Resizes images to target dimensions using configurable strategies.

    Supports letterbox (pad), stretch (distort), and aspect-ratio (fit) modes.
    """

    def __init__(self, config: ResizeConfig | None = None) -> None:
        """Initialize the resizer.

        Args:
            config: Resize configuration. Uses defaults if None.
        """
        self._config = config or ResizeConfig()

    @property
    def config(self) -> ResizeConfig:
        """Get resize config."""
        return self._config

    def resize(self, image: LoadedImage) -> ResizedImage:
        """Resize an image using the configured strategy.

        Args:
            image: The loaded image to resize.

        Returns:
            ResizedImage with resized data and metadata.
        """
        mode = self._config.mode

        if mode == ResizeMode.LETTERBOX:
            return self._resize_letterbox(image)
        elif mode == ResizeMode.STRETCH:
            return self._resize_stretch(image)
        else:
            return self._resize_aspect_ratio(image)

    def _resize_letterbox(self, image: LoadedImage) -> ResizedImage:
        """Resize with letterbox padding to maintain aspect ratio.

        The image is scaled to fit within the target dimensions,
        then centered on a gray canvas of the target size.
        """
        tw, th = self._config.target_width, self._config.target_height
        iw, ih = image.width, image.height

        scale = min(tw / iw, th / ih)
        new_w = int(iw * scale)
        new_h = int(ih * scale)

        resized = cv2.resize(image.data, (new_w, new_h), interpolation=cv2.INTER_LINEAR)

        canvas = np.full(
            (th, tw, image.channels),
            self._config.letterbox_color,
            dtype=np.uint8,
        )

        pad_x = (tw - new_w) // 2
        pad_y = (th - new_h) // 2
        canvas[pad_y : pad_y + new_h, pad_x : pad_x + new_w] = resized

        logger.debug(
            "Letterbox resize %dx%d -> %dx%d (scale=%.3f, pad=%d,%d)",
            iw,
            ih,
            tw,
            th,
            scale,
            pad_x,
            pad_y,
        )

        return ResizedImage(
            data=canvas,
            original_width=iw,
            original_height=ih,
            target_width=tw,
            target_height=th,
            scale=scale,
            pad_x=pad_x,
            pad_y=pad_y,
        )

    def _resize_stretch(self, image: LoadedImage) -> ResizedImage:
        """Resize by stretching to exact target dimensions.

        May distort the aspect ratio.
        """
        tw, th = self._config.target_width, self._config.target_height
        iw, ih = image.width, image.height

        sx = tw / iw
        sy = th / ih

        resized = cv2.resize(image.data, (tw, th), interpolation=cv2.INTER_LINEAR)

        logger.debug("Stretch resize %dx%d -> %dx%d", iw, ih, tw, th)

        return ResizedImage(
            data=resized,
            original_width=iw,
            original_height=ih,
            target_width=tw,
            target_height=th,
            scale=min(sx, sy),
            pad_x=0,
            pad_y=0,
        )

    def _resize_aspect_ratio(self, image: LoadedImage) -> ResizedImage:
        """Resize to fit within target while maintaining aspect ratio.

        No padding is added; the result may be smaller than target.
        """
        tw, th = self._config.target_width, self._config.target_height
        iw, ih = image.width, image.height

        scale = min(tw / iw, th / ih)
        new_w = int(iw * scale)
        new_h = int(ih * scale)

        resized = cv2.resize(image.data, (new_w, new_h), interpolation=cv2.INTER_LINEAR)

        # Pad to exact target size with zeros
        canvas = np.zeros((th, tw, image.channels), dtype=np.uint8)
        canvas[:new_h, :new_w] = resized

        logger.debug(
            "Aspect-ratio resize %dx%d -> %dx%d (scale=%.3f)",
            iw,
            ih,
            new_w,
            new_h,
            scale,
        )

        return ResizedImage(
            data=canvas,
            original_width=iw,
            original_height=ih,
            target_width=tw,
            target_height=th,
            scale=scale,
            pad_x=0,
            pad_y=0,
        )
