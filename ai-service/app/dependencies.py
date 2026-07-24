"""Dependency injection for the application."""

from fastapi import Request

from app.core.metrics import MetricsManager
from app.core.model_manager import ModelManager
from app.services.inference_service import InferenceService


def get_model_manager(request: Request) -> ModelManager:
    """Get model manager from app state.

    Args:
        request: FastAPI request object.

    Returns:
        ModelManager instance.
    """
    return request.app.state.model_manager


def get_metrics_manager(request: Request) -> MetricsManager:
    """Get metrics manager from app state.

    Args:
        request: FastAPI request object.

    Returns:
        MetricsManager instance.
    """
    return request.app.state.metrics_manager


def get_inference_service(request: Request) -> InferenceService:
    """Get inference service from app state.

    Args:
        request: FastAPI request object.

    Returns:
        InferenceService instance.
    """
    return request.app.state.inference_service
