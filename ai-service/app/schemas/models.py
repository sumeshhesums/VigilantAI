"""Model management schemas."""

from pydantic import BaseModel as PydanticBaseModel, ConfigDict, Field


class ModelSummary(PydanticBaseModel):
    """Summary of a registered model."""

    model_config = ConfigDict(protected_namespaces=())

    name: str = Field(..., description="Model name")
    version: str = Field(..., description="Model version")
    model_type: str = Field(..., description="Model architecture type")
    state: str = Field(..., description="Current loading state")
    device: str = Field(..., description="Compute device")
    description: str = Field("", description="Model description")
    loaded: bool = Field(..., description="Whether model is loaded")


class ModelListResponse(PydanticBaseModel):
    """Response containing list of models."""

    models: list[ModelSummary] = Field(
        default_factory=list, description="List of registered models"
    )
    total: int = Field(..., ge=0, description="Total number of models")
    active_model: str | None = Field(None, description="Name of the active model")


class ModelDetailResponse(PydanticBaseModel):
    """Detailed response for a single model."""

    model_config = ConfigDict(protected_namespaces=())

    name: str = Field(..., description="Model name")
    version: str = Field(..., description="Model version")
    model_type: str = Field(..., description="Model architecture type")
    state: str = Field(..., description="Current loading state")
    device: str = Field(..., description="Compute device")
    description: str = Field("", description="Model description")
    loaded: bool = Field(..., description="Whether model is loaded")
    loaded_at: float | None = Field(None, description="Timestamp when loaded")
    load_duration_seconds: float | None = Field(
        None, description="Time taken to load in seconds"
    )
    input_shape: list[int] | None = Field(None, description="Expected input shape")
    class_count: int | None = Field(None, description="Number of output classes")
    error_message: str | None = Field(None, description="Error message if any")
    is_active: bool = Field(False, description="Whether this is the active model")


class ModelActionResponse(PydanticBaseModel):
    """Response for model load/unload/reload actions."""

    name: str = Field(..., description="Model name")
    action: str = Field(..., description="Action performed")
    state: str = Field(..., description="Resulting state")
    message: str = Field("", description="Human-readable status message")
