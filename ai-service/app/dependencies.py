"""Dependency injection for the application."""

from fastapi import Request

from app.core.metrics import MetricsManager
from app.core.model_manager import ModelManager
from app.models.loader import ModelLoader
from app.models.registry import ModelRegistry
from app.services.inference_service import InferenceService


def get_model_manager(request: Request) -> ModelManager:
    """Get model manager from app state."""
    return request.app.state.model_manager


def get_metrics_manager(request: Request) -> MetricsManager:
    """Get metrics manager from app state."""
    return request.app.state.metrics_manager


def get_inference_service(request: Request) -> InferenceService:
    """Get inference service from app state."""
    return request.app.state.inference_service


def get_model_registry(request: Request) -> ModelRegistry:
    """Get model registry from app state."""
    return request.app.state.model_registry


def get_model_loader(request: Request) -> ModelLoader:
    """Get model loader from app state."""
    return request.app.state.model_loader
