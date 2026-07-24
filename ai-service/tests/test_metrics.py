"""Tests for metrics manager."""

import pytest

from app.core.metrics import MetricsManager


class TestMetricsManager:
    """Tests for MetricsManager class."""

    @pytest.fixture
    def manager(self):
        """Create a MetricsManager instance."""
        return MetricsManager()

    def test_initial_state(self, manager: MetricsManager):
        """Test initial metrics state."""
        assert manager.uptime_seconds >= 0
        assert manager.get_request_count() == 0
        assert manager.get_successful_requests() == 0
        assert manager.get_failed_requests() == 0
        assert manager.get_average_inference_time_ms() == 0.0
        assert manager.get_images_processed() == 0
        assert manager.get_total_detections() == 0
        assert manager.get_average_detections_per_image() == 0.0

    def test_record_successful_request(self, manager: MetricsManager):
        """Test recording successful request."""
        manager.record_request(success=True, inference_time_ms=50.0)
        assert manager.get_request_count() == 1
        assert manager.get_successful_requests() == 1
        assert manager.get_failed_requests() == 0

    def test_record_failed_request(self, manager: MetricsManager):
        """Test recording failed request."""
        manager.record_request(success=False, inference_time_ms=100.0)
        assert manager.get_request_count() == 1
        assert manager.get_successful_requests() == 0
        assert manager.get_failed_requests() == 1

    def test_average_inference_time(self, manager: MetricsManager):
        """Test average inference time calculation."""
        manager.record_request(success=True, inference_time_ms=100.0)
        manager.record_request(success=True, inference_time_ms=200.0)
        assert manager.get_average_inference_time_ms() == 150.0

    def test_average_inference_time_no_requests(self, manager: MetricsManager):
        """Test average inference time with no requests."""
        assert manager.get_average_inference_time_ms() == 0.0

    def test_record_inference(self, manager: MetricsManager):
        """Test recording inference with detection count."""
        manager.record_inference(
            success=True, inference_time_ms=45.0, detection_count=3
        )
        assert manager.get_request_count() == 1
        assert manager.get_successful_requests() == 1
        assert manager.get_images_processed() == 1
        assert manager.get_total_detections() == 3

    def test_record_inference_failed(self, manager: MetricsManager):
        """Test recording failed inference."""
        manager.record_inference(
            success=False, inference_time_ms=10.0, detection_count=0
        )
        assert manager.get_failed_requests() == 1
        assert manager.get_images_processed() == 0
        assert manager.get_total_detections() == 0

    def test_average_detections_per_image(self, manager: MetricsManager):
        """Test average detections per image calculation."""
        manager.record_inference(
            success=True, inference_time_ms=50.0, detection_count=2
        )
        manager.record_inference(
            success=True, inference_time_ms=60.0, detection_count=4
        )
        assert manager.get_average_detections_per_image() == 3.0

    def test_get_snapshot(self, manager: MetricsManager):
        """Test get_snapshot returns correct metrics."""
        manager.record_inference(
            success=True, inference_time_ms=50.0, detection_count=2
        )
        manager.record_inference(
            success=False, inference_time_ms=100.0, detection_count=0
        )

        snapshot = manager.get_snapshot()
        assert snapshot.request_count == 2
        assert snapshot.successful_requests == 1
        assert snapshot.failed_requests == 1
        assert snapshot.average_inference_time_ms == 75.0
        assert snapshot.uptime_seconds >= 0
        assert snapshot.images_processed == 1
        assert snapshot.total_detections == 2
        assert snapshot.average_detections_per_image == 2.0

    def test_reset_metrics(self, manager: MetricsManager):
        """Test reset clears all metrics."""
        manager.record_inference(
            success=True, inference_time_ms=50.0, detection_count=3
        )
        manager.reset()

        assert manager.get_request_count() == 0
        assert manager.get_successful_requests() == 0
        assert manager.get_failed_requests() == 0
        assert manager.get_average_inference_time_ms() == 0.0
        assert manager.get_images_processed() == 0
        assert manager.get_total_detections() == 0

    def test_uptime_increases(self, manager: MetricsManager):
        """Test uptime increases over time."""
        import time

        uptime1 = manager.uptime_seconds
        time.sleep(0.01)
        uptime2 = manager.uptime_seconds
        assert uptime2 > uptime1
