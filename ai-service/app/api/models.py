"""Model management API endpoints."""

from fastapi import APIRouter, Depends, HTTPException

from app.dependencies import get_model_manager
from app.logging import get_logger
from app.schemas.models import (
    ModelActionResponse,
    ModelDetailResponse,
    ModelListResponse,
    ModelSummary,
)

router = APIRouter(prefix="/models", tags=["models"])
logger = get_logger(__name__)


@router.get("", response_model=ModelListResponse)
async def list_models(
    model_manager=Depends(get_model_manager),
) -> ModelListResponse:
    """List all registered models.

    Returns:
        ModelListResponse with all registered models and active model info.
    """
    models_data = model_manager.list_models()
    summaries = [
        ModelSummary(
            name=data["name"],
            version=data["version"],
            model_type=data["model_type"],
            state=data["state"],
            device=data["device"],
            description=data.get("description", ""),
            loaded=data["loaded"],
        )
        for data in models_data.values()
    ]
    return ModelListResponse(
        models=summaries,
        total=len(summaries),
        active_model=model_manager.active_model_name,
    )


@router.get("/{name}", response_model=ModelDetailResponse)
async def get_model(
    name: str,
    model_manager=Depends(get_model_manager),
) -> ModelDetailResponse:
    """Get detailed information about a specific model.

    Args:
        name: Model name.

    Returns:
        ModelDetailResponse with full model details.

    Raises:
        HTTPException: 404 if model not found.
    """
    if not model_manager.registry.has_model(name):
        raise HTTPException(status_code=404, detail=f"Model '{name}' not found")

    model = model_manager.registry.get_model(name)
    meta = model.metadata

    return ModelDetailResponse(
        name=meta.name,
        version=meta.version,
        model_type=meta.model_type,
        state=meta.state.value,
        device=meta.device,
        description=meta.description,
        loaded=model.is_loaded,
        loaded_at=meta.loaded_at,
        load_duration_seconds=meta.load_duration_seconds,
        input_shape=meta.input_shape,
        class_count=meta.class_count,
        error_message=meta.error_message,
        is_active=model_manager.active_model_name == name,
    )


@router.post("/{name}/load", response_model=ModelActionResponse)
async def load_model(
    name: str,
    model_manager=Depends(get_model_manager),
) -> ModelActionResponse:
    """Load a specific model.

    Args:
        name: Model name.

    Returns:
        ModelActionResponse with load result.

    Raises:
        HTTPException: 404 if model not found.
    """
    if not model_manager.registry.has_model(name):
        raise HTTPException(status_code=404, detail=f"Model '{name}' not found")

    try:
        await model_manager.load_by_name(name)
        model = model_manager.registry.get_model(name)
        return ModelActionResponse(
            name=name,
            action="load",
            state=model.metadata.state.value,
            message=f"Model '{name}' loaded successfully",
        )
    except Exception as e:
        raise HTTPException(
            status_code=500, detail=f"Failed to load model '{name}': {e}"
        )


@router.post("/{name}/unload", response_model=ModelActionResponse)
async def unload_model(
    name: str,
    model_manager=Depends(get_model_manager),
) -> ModelActionResponse:
    """Unload a specific model.

    Args:
        name: Model name.

    Returns:
        ModelActionResponse with unload result.

    Raises:
        HTTPException: 404 if model not found.
    """
    if not model_manager.registry.has_model(name):
        raise HTTPException(status_code=404, detail=f"Model '{name}' not found")

    try:
        await model_manager.unload_by_name(name)
        model = model_manager.registry.get_model(name)
        return ModelActionResponse(
            name=name,
            action="unload",
            state=model.metadata.state.value,
            message=f"Model '{name}' unloaded successfully",
        )
    except Exception as e:
        raise HTTPException(
            status_code=500, detail=f"Failed to unload model '{name}': {e}"
        )


@router.post("/{name}/reload", response_model=ModelActionResponse)
async def reload_model(
    name: str,
    model_manager=Depends(get_model_manager),
) -> ModelActionResponse:
    """Reload a specific model (unload then load).

    Args:
        name: Model name.

    Returns:
        ModelActionResponse with reload result.

    Raises:
        HTTPException: 404 if model not found.
    """
    if not model_manager.registry.has_model(name):
        raise HTTPException(status_code=404, detail=f"Model '{name}' not found")

    try:
        await model_manager.reload_by_name(name)
        model = model_manager.registry.get_model(name)
        return ModelActionResponse(
            name=name,
            action="reload",
            state=model.metadata.state.value,
            message=f"Model '{name}' reloaded successfully",
        )
    except Exception as e:
        raise HTTPException(
            status_code=500, detail=f"Failed to reload model '{name}': {e}"
        )
