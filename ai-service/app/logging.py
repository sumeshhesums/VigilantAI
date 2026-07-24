"""Structured logging configuration."""

import logging
import sys
from typing import Any


def setup_logging(log_level: str = "INFO") -> None:
    """Configure structured logging for the application.

    Args:
        log_level: The logging level (DEBUG, INFO, WARNING, ERROR, CRITICAL).
    """
    numeric_level = getattr(logging, log_level.upper(), logging.INFO)

    # Configure root logger
    logging.basicConfig(
        level=numeric_level,
        format="%(asctime)s | %(levelname)-8s | %(name)s | %(message)s",
        datefmt="%Y-%m-%d %H:%M:%S",
        handlers=[logging.StreamHandler(sys.stdout)],
        force=True,
    )

    # Suppress noisy third-party loggers
    logging.getLogger("uvicorn.access").setLevel(logging.WARNING)
    logging.getLogger("uvicorn.error").setLevel(logging.INFO)


def get_logger(name: str) -> logging.Logger:
    """Get a logger instance for the given module name.

    Args:
        name: The logger name, typically __name__.

    Returns:
        A configured logger instance.
    """
    return logging.getLogger(name)


class RequestLoggingFilter(logging.Filter):
    """Filter to add request context to log records."""

    def __init__(self, request_id: str | None = None) -> None:
        super().__init__()
        self.request_id = request_id

    def filter(self, record: Any) -> bool:
        record.request_id = self.request_id or "N/A"
        return True
