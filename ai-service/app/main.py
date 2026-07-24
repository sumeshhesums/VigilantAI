"""AI Service - FastAPI Application.

Main application entry point for the VigilantAI AI Service.
Provides object detection inference endpoints.
"""

import time
from contextlib import asynccontextmanager

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.api import health, inference, metrics, models
from app.config import Settings, get_settings
from app.core.metrics import MetricsManager
from app.core.model_manager import ModelManager
from app.inference.detector import YoloDetector
from app.logging import get_logger, setup_logging
from app.models.base import BaseModel
from app.models.factory import GroundingDINOModel, RTDETRModel, YOLOModel
from app.models.loader import ModelLoader
from app.models.registry import ModelRegistry
from app.services.inference_service import InferenceService

logger = get_logger(__name__)

_start_time: float = time.time()

# Map model names to their factory classes
_MODEL_FACTORIES: dict[str, type[BaseModel]] = {
    "yolov8n": YOLOModel,
    "yolov8s": YOLOModel,
    "yolov8m": YOLOModel,
    "yolov8l": YOLOModel,
    "yolov8x": YOLOModel,
    "rtdetr-l": RTDETRModel,
    "rtdetr-x": RTDETRModel,
    "grounding-dino-tiny": GroundingDINOModel,
    "grounding-dino-base": GroundingDINOModel,
}


def _build_registry(settings: Settings) -> ModelRegistry:
    """Build and populate the model registry from settings."""
    registry = ModelRegistry()

    for model_name in settings.AVAILABLE_MODELS:
        factory = _MODEL_FACTORIES.get(model_name)
        if factory is None:
            logger.warning("No factory for model '%s', skipping", model_name)
            continue
        model_instance = factory(name=model_name, device=settings.DEVICE)
        registry.register_model(model_instance)

    if settings.DEFAULT_MODEL in settings.AVAILABLE_MODELS:
        registry.set_default(settings.DEFAULT_MODEL)

    return registry


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Application lifespan manager for startup and shutdown events."""
    settings = get_settings()
    setup_logging(settings.LOG_LEVEL)

    logger.info("Starting AI Service v%s", settings.SERVICE_VERSION)
    logger.info("Default model: %s", settings.DEFAULT_MODEL)
    logger.info("Available models: %s", settings.AVAILABLE_MODELS)

    # Build model infrastructure
    registry = _build_registry(settings)
    loader = ModelLoader(registry)
    model_manager = ModelManager(
        registry=registry,
        loader=loader,
        default_model_name=settings.DEFAULT_MODEL,
    )

    metrics_manager = MetricsManager()

    # Auto-load models if configured
    if settings.AUTO_LOAD and registry.default_model_name:
        logger.info("Auto-loading default model: %s", settings.DEFAULT_MODEL)
        await loader.load_model(settings.DEFAULT_MODEL)

    # Initialize detector and inference service
    detector = YoloDetector(settings=settings)
    inference_service = InferenceService(
        model_manager=model_manager,
        metrics_manager=metrics_manager,
        detector=detector,
    )

    # Store in app state for dependency injection
    app.state.model_registry = registry
    app.state.model_loader = loader
    app.state.model_manager = model_manager
    app.state.metrics_manager = metrics_manager
    app.state.inference_service = inference_service
    app.state.start_time = _start_time

    logger.info("AI Service started successfully")

    yield

    # Shutdown
    logger.info("Shutting down AI Service")
    await loader.unload_all()
    logger.info("AI Service shut down complete")


def create_app() -> FastAPI:
    """Create and configure the FastAPI application."""
    settings = get_settings()

    app = FastAPI(
        title="VigilantAI AI Service",
        description="AI inference service for object detection in security cameras",
        version=settings.SERVICE_VERSION,
        docs_url="/docs",
        redoc_url="/redoc",
        lifespan=lifespan,
    )

    app.add_middleware(
        CORSMiddleware,
        allow_origins=["*"],
        allow_credentials=True,
        allow_methods=["*"],
        allow_headers=["*"],
    )

    app.include_router(health.router)
    app.include_router(inference.router)
    app.include_router(models.router)
    app.include_router(metrics.router)

    return app


app = create_app()
