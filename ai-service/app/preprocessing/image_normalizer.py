"""Image normalizer converting images to model-ready formats."""

from __future__ import annotations

from dataclasses import dataclass
from enum import Enum

import numpy as np

from app.logging import get_logger

logger = get_logger(__name__)

IMAGENET_MEAN = np.array([0.485, 0.456, 0.406], dtype=np.float32)
IMAGENET_STD = np.array([0.229, 0.224, 0.225], dtype=np.float32)


class NormalizationMode(str, Enum):
    """Normalization strategy enumeration."""

    MIN_MAX = "min_max"
    """Scale from [0, 255] to [0, 1]."""

    IMAGENET = "imagenet"
    """Scale to [0, 1] then apply ImageNet mean/std normalization."""

    CUSTOM = "custom"
    """Scale to [0, 1] then apply custom mean/std."""


@dataclass
class NormalizationConfig:
    """Configuration for image normalization."""

    mode: NormalizationMode = NormalizationMode.MIN_MAX
    custom_mean: list[float] | None = None
    """Custom mean values per channel (in 0-1 scale)."""

    custom_std: list[float] | None = None
    """Custom std values per channel (in 0-1 scale)."""

    output_dtype: np.dtype = np.float32
    """Output numpy dtype."""

    def __post_init__(self) -> None:
        if self.mode == NormalizationMode.CUSTOM:
            if self.custom_mean is None or self.custom_std is None:
                raise ValueError("CUSTOM mode requires custom_mean and custom_std")
            if len(self.custom_mean) != len(self.custom_std):
                raise ValueError("custom_mean and custom_std must have same length")


@dataclass
class NormalizedImage:
    """Container for a normalized image."""

    data: np.ndarray
    """Normalized image as float32 NumPy array."""

    mode: NormalizationMode
    """Normalization mode applied."""

    original_dtype: np.dtype
    """Original image dtype before normalization."""

    original_range: tuple[float, float]
    """Original pixel value range (min, max)."""

    @property
    def shape(self) -> tuple[int, ...]:
        """Return array shape."""
        return self.data.shape

    @property
    def dtype(self) -> np.dtype:
        """Return array dtype."""
        return self.data.dtype


class ImageNormalizer:
    """Normalizes images for model input.

    Supports min-max scaling, ImageNet normalization, and custom normalization.
    Always converts to float32.
    """

    def __init__(self, config: NormalizationConfig | None = None) -> None:
        """Initialize the normalizer.

        Args:
            config: Normalization configuration. Uses defaults if None.
        """
        self._config = config or NormalizationConfig()

    @property
    def config(self) -> NormalizationConfig:
        """Get normalization config."""
        return self._config

    def normalize(self, image: np.ndarray) -> NormalizedImage:
        """Normalize an image array.

        Args:
            image: Input image as NumPy array (HWC or HW).

        Returns:
            NormalizedImage with normalized data.
        """
        original_dtype = image.dtype
        original_min = float(image.min())
        original_max = float(image.max())

        float_img = image.astype(np.float32)

        mode = self._config.mode

        if mode == NormalizationMode.MIN_MAX:
            normalized = self._normalize_min_max(float_img)
        elif mode == NormalizationMode.IMAGENET:
            normalized = self._normalize_imagenet(float_img)
        else:
            normalized = self._normalize_custom(float_img)

        logger.debug(
            "Normalized image: mode=%s, shape=%s, range=[%.4f, %.4f]",
            mode.value,
            normalized.shape,
            float(normalized.min()),
            float(normalized.max()),
        )

        return NormalizedImage(
            data=normalized,
            mode=mode,
            original_dtype=original_dtype,
            original_range=(original_min, original_max),
        )

    def _normalize_min_max(self, img: np.ndarray) -> np.ndarray:
        """Scale from [0, 255] to [0, 1]."""
        return img / 255.0

    def _normalize_imagenet(self, img: np.ndarray) -> np.ndarray:
        """Scale to [0, 1] then apply ImageNet mean/std."""
        normalized = img / 255.0

        if len(normalized.shape) == 3 and normalized.shape[2] == 3:
            normalized = (normalized - IMAGENET_MEAN) / IMAGENET_STD

        return normalized

    def _normalize_custom(self, img: np.ndarray) -> np.ndarray:
        """Scale to [0, 1] then apply custom mean/std."""
        normalized = img / 255.0

        mean = np.array(self._config.custom_mean, dtype=np.float32)
        std = np.array(self._config.custom_std, dtype=np.float32)

        if len(normalized.shape) == 3 and normalized.shape[2] == len(mean):
            normalized = (normalized - mean) / std

        return normalized
