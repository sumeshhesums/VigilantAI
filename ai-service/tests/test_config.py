"""Tests for configuration."""

from app.config import Settings, get_settings


class TestSettings:
    """Tests for Settings class."""

    def test_default_settings(self):
        """Test default settings values."""
        settings = Settings()
        assert settings.HOST == "0.0.0.0"
        assert settings.PORT == 8081
        assert settings.LOG_LEVEL == "INFO"
        assert settings.MODEL_NAME == "yolov8n"
        assert settings.MODEL_VERSION == "1.0.0"
        assert settings.DEVICE == "cpu"
        assert settings.MAX_BATCH_SIZE == 16
        assert settings.MAX_IMAGE_SIZE == 1920
        assert settings.CONFIDENCE_THRESHOLD == 0.5
        assert settings.IOU_THRESHOLD == 0.45
        assert settings.MODEL_PATH == ""
        assert settings.INFERENCE_TIMEOUT == 10.0
        assert settings.REQUEST_TIMEOUT == 30.0
        assert settings.SERVICE_NAME == "ai-service"
        assert settings.SERVICE_VERSION == "0.1.0"

    def test_default_model_settings(self):
        """Test default model framework settings."""
        settings = Settings()
        assert settings.DEFAULT_MODEL == "yolov8n"
        assert "yolov8n" in settings.AVAILABLE_MODELS
        assert "rtdetr-l" in settings.AVAILABLE_MODELS
        assert "grounding-dino-tiny" in settings.AVAILABLE_MODELS
        assert settings.AUTO_LOAD is True

    def test_custom_settings(self):
        """Test custom settings override defaults."""
        settings = Settings(
            HOST="127.0.0.1",
            PORT=9000,
            LOG_LEVEL="DEBUG",
            MODEL_NAME="yolov8m",
            DEVICE="cuda",
        )
        assert settings.HOST == "127.0.0.1"
        assert settings.PORT == 9000
        assert settings.LOG_LEVEL == "DEBUG"
        assert settings.MODEL_NAME == "yolov8m"
        assert settings.DEVICE == "cuda"

    def test_custom_model_framework_settings(self):
        """Test custom model framework settings."""
        settings = Settings(
            DEFAULT_MODEL="rtdetr-l",
            AVAILABLE_MODELS=["rtdetr-l", "grounding-dino-tiny"],
            AUTO_LOAD=False,
        )
        assert settings.DEFAULT_MODEL == "rtdetr-l"
        assert settings.AVAILABLE_MODELS == ["rtdetr-l", "grounding-dino-tiny"]
        assert settings.AUTO_LOAD is False

    def test_get_settings_returns_same_instance(self):
        """Test get_settings returns cached instance."""
        settings1 = get_settings()
        settings2 = get_settings()
        assert settings1 is settings2


class TestGetSettings:
    """Tests for get_settings function."""

    def test_returns_settings_instance(self):
        """Test get_settings returns Settings instance."""
        settings = get_settings()
        assert isinstance(settings, Settings)
