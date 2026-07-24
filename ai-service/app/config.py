"""Application configuration using Pydantic Settings."""

from functools import lru_cache

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

    # Model settings
    MODEL_NAME: str = "yolov8n"
    MODEL_VERSION: str = "1.0.0"
    DEVICE: str = "cpu"
    MAX_BATCH_SIZE: int = 16
    MAX_IMAGE_SIZE: int = 1920

    # Request settings
    REQUEST_TIMEOUT: float = 30.0

    # Service metadata
    SERVICE_NAME: str = "ai-service"
    SERVICE_VERSION: str = "0.1.0"


@lru_cache
def get_settings() -> Settings:
    """Get cached settings instance."""
    return Settings()
