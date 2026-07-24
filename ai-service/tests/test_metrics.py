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

    def test_get_snapshot(self, manager: MetricsManager):
        """Test get_snapshot returns correct metrics."""
        manager.record_request(success=True, inference_time_ms=50.0)
        manager.record_request(success=False, inference_time_ms=100.0)

        snapshot = manager.get_snapshot()
        assert snapshot.request_count == 2
        assert snapshot.successful_requests == 1
        assert snapshot.failed_requests == 1
        assert snapshot.average_inference_time_ms == 75.0
        assert snapshot.uptime_seconds >= 0

    def test_reset_metrics(self, manager: MetricsManager):
        """Test reset clears all metrics."""
        manager.record_request(success=True, inference_time_ms=50.0)
        manager.reset()

        assert manager.get_request_count() == 0
        assert manager.get_successful_requests() == 0
        assert manager.get_failed_requests() == 0
        assert manager.get_average_inference_time_ms() == 0.0

    def test_uptime_increases(self, manager: MetricsManager):
        """Test uptime increases over time."""
        import time

        uptime1 = manager.uptime_seconds
        time.sleep(0.01)
        uptime2 = manager.uptime_seconds
        assert uptime2 > uptime1
