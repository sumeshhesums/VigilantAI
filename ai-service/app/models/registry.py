"""Model registry for managing available models."""

from typing import Any

from app.logging import get_logger
from app.models.base import BaseModel

logger = get_logger(__name__)


class ModelRegistry:
    """Central registry for AI model instances.

    Manages registration, lookup, and enumeration of models.
    Thread-safe via GIL for dict operations, but callers should
    use async locks if concurrent modification is expected.
    """

    def __init__(self) -> None:
        self._models: dict[str, BaseModel] = {}
        self._default_model_name: str | None = None

    def register_model(self, model: BaseModel) -> None:
        """Register a model in the registry.

        Args:
            model: Model instance to register.

        Raises:
            ValueError: If a model with the same name is already registered.
        """
        name = model.name
        if name in self._models:
            raise ValueError(f"Model '{name}' is already registered")
        self._models[name] = model
        logger.info("Registered model: %s", name)

    def unregister_model(self, name: str) -> BaseModel:
        """Remove a model from the registry.

        Args:
            name: Name of the model to remove.

        Returns:
            The removed model instance.

        Raises:
            KeyError: If no model with that name is registered.
        """
        if name not in self._models:
            raise KeyError(f"Model '{name}' not found in registry")
        model = self._models.pop(name)
        if self._default_model_name == name:
            self._default_model_name = None
        logger.info("Unregistered model: %s", name)
        return model

    def get_model(self, name: str) -> BaseModel:
        """Get a model by name.

        Args:
            name: Name of the model.

        Returns:
            The model instance.

        Raises:
            KeyError: If no model with that name is registered.
        """
        try:
            return self._models[name]
        except KeyError:
            raise KeyError(f"Model '{name}' not found in registry")

    def has_model(self, name: str) -> bool:
        """Check if a model is registered.

        Args:
            name: Name of the model.

        Returns:
            True if registered.
        """
        return name in self._models

    def list_models(self) -> dict[str, dict[str, Any]]:
        """List all registered models with their metadata.

        Returns:
            Dictionary mapping model names to their status info.
        """
        return {
            name: {
                "name": model.metadata.name,
                "version": model.metadata.version,
                "model_type": model.metadata.model_type,
                "state": model.metadata.state.value,
                "device": model.metadata.device,
                "description": model.metadata.description,
                "loaded": model.is_loaded,
            }
            for name, model in self._models.items()
        }

    def list_model_names(self) -> list[str]:
        """List all registered model names.

        Returns:
            Sorted list of model names.
        """
        return sorted(self._models.keys())

    def set_default(self, name: str) -> None:
        """Set the default model.

        Args:
            name: Name of the model to set as default.

        Raises:
            KeyError: If no model with that name is registered.
        """
        if name not in self._models:
            raise KeyError(f"Model '{name}' not found in registry")
        self._default_model_name = name
        logger.info("Default model set to: %s", name)

    def default_model(self) -> BaseModel | None:
        """Get the default model.

        Returns:
            The default model instance, or None if not set.
        """
        if self._default_model_name is None:
            return None
        return self._models.get(self._default_model_name)

    @property
    def default_model_name(self) -> str | None:
        """Get the default model name."""
        return self._default_model_name

    @property
    def count(self) -> int:
        """Get the number of registered models."""
        return len(self._models)
