# Written by Luis Felipe Montemayor, sometime around January of 2026
import os

from glia_common.logs import setup_logger

import glia_core
from glia_python.JobMetrics import JobMetrics

logger = setup_logger("glia_python")


def push_telemetry(
    metrics: JobMetrics, api_url: str | None = None, timeout: float = 2.0
) -> bool:
    """
    - Auto-config: Reads API_INGEST_URL env var if api_url is not provided.
    - Suppress Errors: Returns False on failure instead of crashing (Fail Silent).
    - Non-blocking: Enqueues the telemetry to the Rust background worker.
    """
    target_url: str | None = api_url or os.getenv("API_INGEST_URL")
    if not target_url:
        logger.warning("[Glia] No API_INGEST_URL configured. Telemetry dropped.")
        return False

    try:
        json_payload = metrics.model_dump_json()
        glia_core.queue_telemetry(json_payload, target_url, timeout)
        return True

    except Exception as e:
        logger.warning(f"[Glia] Unexpected error during telemetry queuing: {e}")
        return False
