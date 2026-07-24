"""Model loader for managing model lifecycle operations."""

from typing import Any

from app.logging import get_logger
from app.models.registry import ModelRegistry

logger = get_logger(__name__)


class ModelLoader:
    """Orchestrates model loading across the registry.

    Coordinates load/unload/reload operations and provides
    bulk operations for managing all registered models.
    """

    def __init__(self, registry: ModelRegistry) -> None:
        self._registry = registry

    async def load_model(self, name: str) -> dict[str, Any]:
        """Load a specific model by name.

        Args:
            name: Name of the model to load.

        Returns:
            Model metadata dictionary.

        Raises:
            KeyError: If model not found in registry.
        """
        model = self._registry.get_model(name)
        metadata = await model.load()
        return metadata.to_dict()

    async def unload_model(self, name: str) -> None:
        """Unload a specific model by name.

        Args:
            name: Name of the model to unload.

        Raises:
            KeyError: If model not found in registry.
        """
        model = self._registry.get_model(name)
        await model.unload()

    async def reload_model(self, name: str) -> dict[str, Any]:
        """Reload a specific model by name.

        Args:
            name: Name of the model to reload.

        Returns:
            Model metadata dictionary after reload.

        Raises:
            KeyError: If model not found in registry.
        """
        model = self._registry.get_model(name)
        await model.unload()
        metadata = await model.load()
        return metadata.to_dict()

    async def load_all(self) -> dict[str, dict[str, Any]]:
        """Load all registered models.

        Returns:
            Dictionary mapping model names to their metadata.
        """
        results: dict[str, dict[str, Any]] = {}
        for name in self._registry.list_model_names():
            try:
                metadata = await self.load_model(name)
                results[name] = metadata
            except Exception as e:
                logger.error("Failed to load model %s: %s", name, e)
                results[name] = {"error": str(e)}
        return results

    async def unload_all(self) -> None:
        """Unload all registered models."""
        for name in self._registry.list_model_names():
            try:
                await self.unload_model(name)
            except Exception as e:
                logger.error("Failed to unload model %s: %s", name, e)

    async def warmup_model(self, name: str) -> dict[str, Any]:
        """Warm up a specific model by name.

        Args:
            name: Name of the model to warm up.

        Returns:
            Model metadata dictionary after warmup.

        Raises:
            KeyError: If model not found in registry.
            RuntimeError: If model is not loaded.
        """
        model = self._registry.get_model(name)
        metadata = await model.warmup()
        return metadata.to_dict()

    def get_model_status(self, name: str) -> dict[str, Any]:
        """Get status of a specific model.

        Args:
            name: Name of the model.

        Returns:
            Model health dictionary.

        Raises:
            KeyError: If model not found in registry.
        """
        model = self._registry.get_model(name)
        return model.health()

    def get_all_statuses(self) -> dict[str, dict[str, Any]]:
        """Get status of all registered models.

        Returns:
            Dictionary mapping model names to health info.
        """
        return {name: model.health() for name, model in self._registry._models.items()}
