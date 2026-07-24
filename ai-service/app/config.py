"""Application configuration using Pydantic Settings."""

from functools import lru_cache

from pydantic import Field
from pydantic_settings import BaseSettings, SettingsConfigDict


class Settings(BaseSettings):
    """Application settings loaded from environment variables."""

    model_config = SettingsConfigDict(
        env_prefix="AI_SERVICE_",
        env_file=".env",
        env_file_encoding="utf-8",
        case_sensitive=False,
    )

    # Server settings
    HOST: str = "0.0.0.0"
    PORT: int = 8081
    LOG_LEVEL: str = "INFO"
    DEBUG: bool = False

    # Model settings (legacy single-model)
    MODEL_NAME: str = "yolov8n"
    MODEL_VERSION: str = "1.0.0"
    DEVICE: str = "cpu"
    MAX_BATCH_SIZE: int = 16
    MAX_IMAGE_SIZE: int = 1920

    # Multi-model framework settings
    DEFAULT_MODEL: str = "yolov8n"
    AVAILABLE_MODELS: list[str] = Field(
        default_factory=lambda: ["yolov8n", "rtdetr-l", "grounding-dino-tiny"]
    )
    AUTO_LOAD: bool = True

    # Preprocessing settings
    IMAGE_SIZE: list[int] = Field(
        default_factory=lambda: [640, 640],
        description="Target image dimensions [width, height]",
    )
    NORMALIZATION: str = Field(
        default="min_max",
        description="Normalization mode: min_max, imagenet, custom",
    )
    MAX_IMAGE_PIXELS: int = Field(
        default=100_000_000,
        description="Maximum allowed pixel count",
    )

    # Request settings
    REQUEST_TIMEOUT: float = 30.0

    # Service metadata
    SERVICE_NAME: str = "ai-service"
    SERVICE_VERSION: str = "0.1.0"


@lru_cache
def get_settings() -> Settings:
    """Get cached settings instance."""
    return Settings()
