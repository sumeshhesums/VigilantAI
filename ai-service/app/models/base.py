"""Abstract base model for AI inference."""

import time
from abc import ABC, abstractmethod
from dataclasses import dataclass, field
from enum import Enum
from typing import Any

from app.logging import get_logger

logger = get_logger(__name__)


class ModelState(str, Enum):
    """Model loading state."""

    NOT_LOADED = "not_loaded"
    LOADING = "loading"
    LOADED = "loaded"
    WARMING_UP = "warming_up"
    ERROR = "error"


@dataclass
class ModelMetadata:
    """Metadata describing an AI model."""

    name: str
    version: str
    model_type: str
    device: str = "cpu"
    state: ModelState = ModelState.NOT_LOADED
    loaded_at: float | None = None
    load_duration_seconds: float | None = None
    input_shape: list[int] | None = None
    class_count: int | None = None
    description: str = ""
    error_message: str | None = None
    extra: dict[str, Any] = field(default_factory=dict)

    def to_dict(self) -> dict[str, Any]:
        """Serialize metadata to dictionary."""
        return {
            "name": self.name,
            "version": self.version,
            "model_type": self.model_type,
            "device": self.device,
            "state": self.state.value,
            "loaded_at": self.loaded_at,
            "load_duration_seconds": self.load_duration_seconds,
            "input_shape": self.input_shape,
            "class_count": self.class_count,
            "description": self.description,
            "error_message": self.error_message,
            "extra": self.extra,
        }


class BaseModel(ABC):
    """Abstract base class for all AI models.

    Subclasses must implement load_model and unload_model.
    The base class handles lifecycle state tracking.
    """

    def __init__(
        self,
        name: str,
        version: str,
        model_type: str,
        device: str = "cpu",
        description: str = "",
    ) -> None:
        self._metadata = ModelMetadata(
            name=name,
            version=version,
            model_type=model_type,
            device=device,
            description=description,
        )

    @property
    def metadata(self) -> ModelMetadata:
        """Get current model metadata."""
        return self._metadata

    @property
    def name(self) -> str:
        """Get model name."""
        return self._metadata.name

    @property
    def is_loaded(self) -> bool:
        """Check if model is loaded and ready."""
        return self._metadata.state == ModelState.LOADED

    @abstractmethod
    async def load_model(self) -> None:
        """Load the model into memory.

        Must set self._metadata.state to LOADED on success
        or ERROR on failure.
        """

    @abstractmethod
    async def unload_model(self) -> None:
        """Unload the model from memory.

        Must set self._metadata.state to NOT_LOADED.
        """

    async def load(self) -> ModelMetadata:
        """Load the model with lifecycle tracking.

        Returns:
            Updated ModelMetadata after load attempt.
        """
        if self._metadata.state == ModelState.LOADED:
            logger.info("Model %s already loaded", self._metadata.name)
            return self._metadata

        self._metadata.state = ModelState.LOADING
        self._metadata.error_message = None
        start_time = time.time()

        try:
            await self.load_model()
            self._metadata.state = ModelState.LOADED
            self._metadata.loaded_at = time.time()
            self._metadata.load_duration_seconds = time.time() - start_time
            logger.info(
                "Model %s loaded in %.2fs",
                self._metadata.name,
                self._metadata.load_duration_seconds,
            )
        except Exception as e:
            self._metadata.state = ModelState.ERROR
            self._metadata.error_message = str(e)
            logger.error("Failed to load model %s: %s", self._metadata.name, e)

        return self._metadata

    async def unload(self) -> None:
        """Unload the model with lifecycle tracking."""
        if self._metadata.state == ModelState.NOT_LOADED:
            return

        try:
            await self.unload_model()
        except Exception as e:
            logger.error("Error unloading model %s: %s", self._metadata.name, e)

        self._metadata.state = ModelState.NOT_LOADED
        self._metadata.loaded_at = None
        self._metadata.load_duration_seconds = None
        self._metadata.error_message = None
        logger.info("Model %s unloaded", self._metadata.name)

    async def warmup(self) -> ModelMetadata:
        """Warm up the model by performing a dummy inference pass.

        Default implementation marks state as WARMING_UP then LOADED.
        Subclasses can override for model-specific warmup logic.
        """
        if not self.is_loaded:
            raise RuntimeError(f"Model {self._metadata.name} must be loaded first")

        self._metadata.state = ModelState.WARMING_UP
        logger.info("Warming up model %s", self._metadata.name)

        # Placeholder: In production, run a dummy inference
        import asyncio

        await asyncio.sleep(0.05)

        self._metadata.state = ModelState.LOADED
        logger.info("Model %s warmup complete", self._metadata.name)
        return self._metadata

    def health(self) -> dict[str, Any]:
        """Report model health status.

        Returns:
            Dictionary with health information.
        """
        return {
            "name": self._metadata.name,
            "state": self._metadata.state.value,
            "loaded": self.is_loaded,
            "device": self._metadata.device,
            "error": self._metadata.error_message,
        }
