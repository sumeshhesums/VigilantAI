"""Image validator for format, dimensions, and content checks."""

from __future__ import annotations

from dataclasses import dataclass, field


from app.logging import get_logger
from app.preprocessing.image_loader import LoadedImage

logger = get_logger(__name__)


class ValidationError(Exception):
    """Raised when image validation fails."""


@dataclass
class ValidationConfig:
    """Configuration for image validation."""

    min_width: int = 1
    min_height: int = 1
    max_width: int = 10_000
    max_height: int = 10_000
    max_pixels: int = 100_000_000
    require_rgb: bool = True
    supported_channels: tuple[int, ...] = (1, 3, 4)


@dataclass
class ValidationResult:
    """Result of image validation."""

    is_valid: bool
    errors: list[str] = field(default_factory=list)
    warnings: list[str] = field(default_factory=list)

    def add_error(self, message: str) -> None:
        """Add an error and mark invalid."""
        self.errors.append(message)
        self.is_valid = False

    def add_warning(self, message: str) -> None:
        """Add a warning."""
        self.warnings.append(message)


class ImageValidator:
    """Validates loaded images against configurable constraints.

    Checks: format, dimensions, channel count, empty content.
    Can optionally convert to RGB.
    """

    def __init__(self, config: ValidationConfig | None = None) -> None:
        """Initialize the validator.

        Args:
            config: Validation configuration. Uses defaults if None.
        """
        self._config = config or ValidationConfig()

    @property
    def config(self) -> ValidationConfig:
        """Get validation config."""
        return self._config

    def validate(self, image: LoadedImage) -> ValidationResult:
        """Validate a loaded image.

        Args:
            image: The loaded image to validate.

        Returns:
            ValidationResult with is_valid, errors, and warnings.
        """
        result = ValidationResult(is_valid=True)

        self._check_empty(image, result)
        self._check_dimensions(image, result)
        self._check_pixels(image, result)
        self._check_channels(image, result)

        if result.is_valid:
            logger.debug("Image %s passed validation", image.source)
        else:
            logger.warning(
                "Image %s failed validation: %s",
                image.source,
                "; ".join(result.errors),
            )

        return result

    def validate_or_raise(self, image: LoadedImage) -> None:
        """Validate and raise on failure.

        Args:
            image: The loaded image to validate.

        Raises:
            ValidationError: If validation fails.
        """
        result = self.validate(image)
        if not result.is_valid:
            raise ValidationError(
                f"Image validation failed for {image.source}: "
                + "; ".join(result.errors)
            )

    def to_rgb(self, image: LoadedImage) -> LoadedImage:
        """Convert image to RGB if it is BGR.

        Args:
            image: The loaded image (expected BGR).

        Returns:
            New LoadedImage with RGB data, or original if already RGB/grayscale.
        """
        if image.channels != 3:
            return image

        import cv2

        rgb_data = cv2.cvtColor(image.data, cv2.COLOR_BGR2RGB)
        return LoadedImage(
            data=rgb_data,
            source=image.source,
            width=image.width,
            height=image.height,
            channels=image.channels,
        )

    def _check_empty(self, image: LoadedImage, result: ValidationResult) -> None:
        """Check if image data is empty."""
        if image.data.size == 0:
            result.add_error("Image data is empty (zero elements)")

    def _check_dimensions(self, image: LoadedImage, result: ValidationResult) -> None:
        """Check image dimensions against configured bounds."""
        cfg = self._config

        if image.width < cfg.min_width or image.height < cfg.min_height:
            result.add_error(
                f"Image too small: {image.width}x{image.height}, "
                f"minimum: {cfg.min_width}x{cfg.min_height}"
            )

        if image.width > cfg.max_width or image.height > cfg.max_height:
            result.add_error(
                f"Image too large: {image.width}x{image.height}, "
                f"maximum: {cfg.max_width}x{cfg.max_height}"
            )

        if image.width * image.height > cfg.max_pixels:
            result.add_error(
                f"Image pixel count {image.width * image.height:,} "
                f"exceeds maximum {cfg.max_pixels:,}"
            )

    def _check_pixels(self, image: LoadedImage, result: ValidationResult) -> None:
        """Check for all-zero (black) or degenerate images."""
        if image.data.sum() == 0:
            result.add_warning("Image is completely black (all pixels are zero)")

    def _check_channels(self, image: LoadedImage, result: ValidationResult) -> None:
        """Check channel count."""
        cfg = self._config

        if image.channels not in cfg.supported_channels:
            result.add_error(
                f"Unsupported channel count: {image.channels}, "
                f"supported: {cfg.supported_channels}"
            )

        if cfg.require_rgb and image.channels not in (1, 3):
            result.add_error(
                f"Expected 1 or 3 channels (require_rgb=True), " f"got {image.channels}"
            )
