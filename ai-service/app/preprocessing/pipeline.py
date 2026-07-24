"""Preprocessing pipeline combining load, validate, resize, normalize."""

from __future__ import annotations

from dataclasses import dataclass, field
from pathlib import Path
from typing import Union

import numpy as np

from app.logging import get_logger
from app.preprocessing.image_loader import ImageLoader, LoadedImage
from app.preprocessing.image_normalizer import (
    ImageNormalizer,
    NormalizationConfig,
)
from app.preprocessing.image_resizer import (
    ImageResizer,
    ResizeConfig,
)
from app.preprocessing.image_validator import (
    ImageValidator,
    ValidationConfig,
)

logger = get_logger(__name__)


@dataclass
class PipelineConfig:
    """Configuration for the full preprocessing pipeline."""

    validation: ValidationConfig = field(default_factory=ValidationConfig)
    resize: ResizeConfig = field(default_factory=ResizeConfig)
    normalization: NormalizationConfig = field(default_factory=NormalizationConfig)
    convert_to_rgb: bool = True


@dataclass
class PreprocessedImage:
    """Final output of the preprocessing pipeline."""

    data: np.ndarray
    """Preprocessed image as float32 NumPy array (HWC)."""

    original_width: int
    """Original image width."""

    original_height: int
    """Original image height."""

    resized_width: int
    """Width after resize."""

    resized_height: int
    """Height after resize."""

    scale: float
    """Scale factor applied during resize."""

    pad_x: int
    """Horizontal padding added."""

    pad_y: int
    """Vertical padding added."""

    source: str
    """Original image source identifier."""

    processing_time_ms: float
    """Total pipeline processing time in milliseconds."""

    @property
    def shape(self) -> tuple[int, ...]:
        """Return array shape."""
        return self.data.shape

    @property
    def dtype(self) -> np.dtype:
        """Return array dtype."""
        return self.data.dtype


class PreprocessingPipeline:
    """Full image preprocessing pipeline.

    Orchestrates: load -> validate -> resize -> normalize -> return.
    """

    def __init__(self, config: PipelineConfig | None = None) -> None:
        """Initialize the pipeline.

        Args:
            config: Pipeline configuration. Uses defaults if None.
        """
        self._config = config or PipelineConfig()
        self._loader = ImageLoader(use_bgr=not self._config.convert_to_rgb)
        self._validator = ImageValidator(self._config.validation)
        self._resizer = ImageResizer(self._config.resize)
        self._normalizer = ImageNormalizer(self._config.normalization)

    @property
    def config(self) -> PipelineConfig:
        """Get pipeline config."""
        return self._config

    @property
    def loader(self) -> ImageLoader:
        """Access the image loader."""
        return self._loader

    @property
    def validator(self) -> ImageValidator:
        """Access the image validator."""
        return self._validator

    @property
    def resizer(self) -> ImageResizer:
        """Access the image resizer."""
        return self._resizer

    @property
    def normalizer(self) -> ImageNormalizer:
        """Access the image normalizer."""
        return self._normalizer

    def process_file(self, path: Union[str, Path]) -> PreprocessedImage:
        """Process an image file through the full pipeline.

        Args:
            path: Path to the image file.

        Returns:
            PreprocessedImage with the final result.

        Raises:
            FileNotFoundError: If file does not exist.
            ValidationError: If validation fails.
            RuntimeError: If processing fails.
        """
        import time

        start = time.perf_counter()

        image = self._loader.load_from_file(path)
        return self._run_pipeline(image, start)

    def process_bytes(
        self,
        data: bytes,
        source: str = "<bytes>",
    ) -> PreprocessedImage:
        """Process raw image bytes through the full pipeline.

        Args:
            data: Raw image bytes.
            source: Identifier for error messages.

        Returns:
            PreprocessedImage with the final result.

        Raises:
            ValueError: If data is empty.
            ValidationError: If validation fails.
            RuntimeError: If processing fails.
        """
        import time

        start = time.perf_counter()

        image = self._loader.load_from_bytes(data, source=source)
        return self._run_pipeline(image, start)

    def process_image(self, image: LoadedImage) -> PreprocessedImage:
        """Process a pre-loaded image through validate/resize/normalize.

        Args:
            image: Already loaded image.

        Returns:
            PreprocessedImage with the final result.

        Raises:
            ValidationError: If validation fails.
        """
        import time

        start = time.perf_counter()
        return self._run_pipeline(image, start)

    def _run_pipeline(
        self,
        image: LoadedImage,
        start_time: float,
    ) -> PreprocessedImage:
        """Execute the pipeline steps.

        Args:
            image: Loaded image to process.
            start_time: Pipeline start time for timing.

        Returns:
            PreprocessedImage with the result.
        """
        import time

        # Step 1: Validate
        self._validator.validate_or_raise(image)

        # Step 2: Convert to RGB if needed
        if self._config.convert_to_rgb and image.channels == 3:
            image = self._validator.to_rgb(image)

        # Step 3: Resize
        resized = self._resizer.resize(image)

        # Step 4: Normalize
        normalized = self._normalizer.normalize(resized.data)

        elapsed_ms = (time.perf_counter() - start_time) * 1000

        logger.info(
            "Preprocessed %s: %dx%d -> %dx%d in %.2fms",
            image.source,
            image.width,
            image.height,
            resized.width,
            resized.height,
            elapsed_ms,
        )

        return PreprocessedImage(
            data=normalized.data,
            original_width=image.width,
            original_height=image.height,
            resized_width=resized.width,
            resized_height=resized.height,
            scale=resized.scale,
            pad_x=resized.pad_x,
            pad_y=resized.pad_y,
            source=image.source,
            processing_time_ms=elapsed_ms,
        )
