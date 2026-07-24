"""Metrics manager for tracking service performance."""

import time
from dataclasses import dataclass


@dataclass
class MetricsSnapshot:
    """Point-in-time snapshot of service metrics."""

    request_count: int
    successful_requests: int
    failed_requests: int
    average_inference_time_ms: float
    uptime_seconds: float


class MetricsManager:
    """Tracks service metrics and performance statistics."""

    def __init__(self) -> None:
        self._start_time: float = time.time()
        self._request_count: int = 0
        self._successful_requests: int = 0
        self._failed_requests: int = 0
        self._total_inference_time_ms: float = 0.0

    @property
    def uptime_seconds(self) -> float:
        """Get service uptime in seconds."""
        return time.time() - self._start_time

    def record_request(self, success: bool, inference_time_ms: float) -> None:
        """Record a request metric.

        Args:
            success: Whether the request was successful.
            inference_time_ms: Inference time in milliseconds.
        """
        self._request_count += 1
        if success:
            self._successful_requests += 1
        else:
            self._failed_requests += 1
        self._total_inference_time_ms += inference_time_ms

    def get_request_count(self) -> int:
        """Get total request count."""
        return self._request_count

    def get_successful_requests(self) -> int:
        """Get successful request count."""
        return self._successful_requests

    def get_failed_requests(self) -> int:
        """Get failed request count."""
        return self._failed_requests

    def get_average_inference_time_ms(self) -> float:
        """Get average inference time in milliseconds."""
        if self._request_count == 0:
            return 0.0
        return self._total_inference_time_ms / self._request_count

    def get_snapshot(self) -> MetricsSnapshot:
        """Get current metrics snapshot.

        Returns:
            MetricsSnapshot with current metrics.
        """
        return MetricsSnapshot(
            request_count=self._request_count,
            successful_requests=self._successful_requests,
            failed_requests=self._failed_requests,
            average_inference_time_ms=self.get_average_inference_time_ms(),
            uptime_seconds=self.uptime_seconds,
        )

    def reset(self) -> None:
        """Reset all metrics."""
        self._start_time = time.time()
        self._request_count = 0
        self._successful_requests = 0
        self._failed_requests = 0
        self._total_inference_time_ms = 0.0
