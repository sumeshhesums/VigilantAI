"""Model manager for AI inference models.

Supports multiple models with an active model concept.
"""

from typing import Any

from app.logging import get_logger
from app.models.base import BaseModel, ModelMetadata, ModelState
from app.models.loader import ModelLoader
from app.models.registry import ModelRegistry

logger = get_logger(__name__)


class ModelManager:
    """High-level model manager supporting multiple models.

    Wraps ModelRegistry and ModelLoader to provide a unified
    interface for multi-model management with active model tracking.
    """

    def __init__(
        self,
        registry: ModelRegistry,
        loader: ModelLoader,
        default_model_name: str | None = None,
    ) -> None:
        self._registry = registry
        self._loader = loader
        self._active_model_name: str | None = default_model_name

    @property
    def active_model_name(self) -> str | None:
        """Get the name of the currently active model."""
        return self._active_model_name

    @property
    def active_model(self) -> BaseModel | None:
        """Get the currently active model instance."""
        if self._active_model_name is None:
            return None
        try:
            return self._registry.get_model(self._active_model_name)
        except KeyError:
            return None

    @property
    def registry(self) -> ModelRegistry:
        """Access the underlying model registry."""
        return self._registry

    @property
    def loader(self) -> ModelLoader:
        """Access the underlying model loader."""
        return self._loader

    def set_active_model(self, name: str) -> None:
        """Set the active model by name.

        Args:
            name: Name of the model to set as active.

        Raises:
            KeyError: If model not found in registry.
        """
        if not self._registry.has_model(name):
            raise KeyError(f"Model '{name}' not found in registry")
        self._active_model_name = name
        logger.info("Active model set to: %s", name)

    async def switch_model(self, name: str) -> dict[str, Any]:
        """Switch to a different model.

        Loads the target model if not already loaded, then sets it active.

        Args:
            name: Name of the model to switch to.

        Returns:
            Metadata dictionary of the new active model.

        Raises:
            KeyError: If model not found in registry.
        """
        if not self._registry.has_model(name):
            raise KeyError(f"Model '{name}' not found in registry")

        model = self._registry.get_model(name)
        if not model.is_loaded:
            await model.load()

        self._active_model_name = name
        logger.info("Switched active model to: %s", name)
        return model.metadata.to_dict()

    async def load_by_name(self, name: str) -> dict[str, Any]:
        """Load a model by name.

        Args:
            name: Name of the model to load.

        Returns:
            Model metadata dictionary.
        """
        return await self._loader.load_model(name)

    async def unload_by_name(self, name: str) -> None:
        """Unload a model by name.

        If this is the active model, clears the active model.

        Args:
            name: Name of the model to unload.
        """
        await self._loader.unload_model(name)
        if self._active_model_name == name:
            self._active_model_name = None
            logger.info("Cleared active model (was %s)", name)

    async def reload_by_name(self, name: str) -> dict[str, Any]:
        """Reload a model by name.

        Args:
            name: Name of the model to reload.

        Returns:
            Model metadata dictionary after reload.
        """
        return await self._loader.reload_model(name)

    def get_status(self, name: str) -> dict[str, Any]:
        """Get status of a specific model.

        Args:
            name: Name of the model.

        Returns:
            Model status dictionary.
        """
        return self._loader.get_model_status(name)

    def get_all_statuses(self) -> dict[str, dict[str, Any]]:
        """Get status of all registered models.

        Returns:
            Dictionary mapping model names to status.
        """
        return self._loader.get_all_statuses()

    def list_models(self) -> dict[str, dict[str, Any]]:
        """List all registered models with metadata.

        Returns:
            Dictionary mapping model names to metadata.
        """
        return self._registry.list_models()

    # --- Legacy single-model interface for backward compatibility ---

    def get_model_info(self) -> dict[str, Any]:
        """Get active model info for health checks.

        Returns:
            Dictionary with model info, falling back to defaults.
        """
        model = self.active_model
        if model is not None:
            return {
                "name": model.metadata.name,
                "version": model.metadata.version,
                "status": model.metadata.state.value,
                "device": model.metadata.device,
                "input_shape": model.metadata.input_shape,
                "class_count": model.metadata.class_count,
            }
        return {
            "name": "unknown",
            "version": "0.0.0",
            "status": ModelState.NOT_LOADED.value,
            "device": "cpu",
            "input_shape": None,
            "class_count": None,
        }

    @property
    def is_loaded(self) -> bool:
        """Check if the active model is loaded."""
        model = self.active_model
        return model is not None and model.is_loaded

    @property
    def metadata(self) -> ModelMetadata:
        """Get active model metadata (legacy compat)."""
        model = self.active_model
        if model is not None:
            return model.metadata
        return ModelMetadata(
            name="unknown",
            version="0.0.0",
            model_type="unknown",
        )
