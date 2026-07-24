"""AI Service - FastAPI Application.

Main application entry point for the VigilantAI AI Service.
Provides object detection inference endpoints.
"""

import time
from contextlib import asynccontextmanager

from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from app.api import health, inference
from app.config import get_settings
from app.core.metrics import MetricsManager
from app.core.model_manager import ModelManager
from app.logging import get_logger, setup_logging
from app.services.inference_service import InferenceService

logger = get_logger(__name__)

# Application start time for uptime tracking
_start_time: float = time.time()


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Application lifespan manager for startup and shutdown events."""
    settings = get_settings()
    setup_logging(settings.LOG_LEVEL)

    logger.info("Starting AI Service v%s", settings.SERVICE_VERSION)
    logger.info(
        "Model: %s v%s on %s",
        settings.MODEL_NAME,
        settings.MODEL_VERSION,
        settings.DEVICE,
    )

    # Initialize managers
    model_manager = ModelManager(
        model_name=settings.MODEL_NAME,
        model_version=settings.MODEL_VERSION,
        device=settings.DEVICE,
    )
    metrics_manager = MetricsManager()

    # Load the model
    await model_manager.load_model()

    # Initialize inference service
    inference_service = InferenceService(
        model_manager=model_manager,
        metrics_manager=metrics_manager,
    )

    # Store in app state for dependency injection
    app.state.model_manager = model_manager
    app.state.metrics_manager = metrics_manager
    app.state.inference_service = inference_service
    app.state.start_time = _start_time

    logger.info("AI Service started successfully")

    yield

    # Shutdown
    logger.info("Shutting down AI Service")
    await model_manager.unload_model()
    logger.info("AI Service shut down complete")


def create_app() -> FastAPI:
    """Create and configure the FastAPI application.

    Returns:
        Configured FastAPI application instance.
    """
    settings = get_settings()

    app = FastAPI(
        title="VigilantAI AI Service",
        description="AI inference service for object detection in security cameras",
        version=settings.SERVICE_VERSION,
        docs_url="/docs",
        redoc_url="/redoc",
        lifespan=lifespan,
    )

    # CORS middleware
    app.add_middleware(
        CORSMiddleware,
        allow_origins=["*"],
        allow_credentials=True,
        allow_methods=["*"],
        allow_headers=["*"],
    )

    # Include routers
    app.include_router(health.router)
    app.include_router(inference.router)

    return app


app = create_app()
