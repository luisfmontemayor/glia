# Written by Luis Felipe Montemayor, sometime around January of 2026
import os

from common.logs import setup_logger

import core
from glia_python.JobMetrics import JobMetrics

logger = setup_logger("glia_python")


def push_telemetry(
    metrics: JobMetrics, api_url: str | None = None, timeout: float | None = None
) -> bool:
    """
    - Auto-config: Reads API_INGEST_URL env var if api_url is not provided.
    - Suppress Errors: Returns False on failure instead of crashing (Fail Silent).
    - Non-blocking: Enqueues the telemetry to the Rust background worker.
    """
    target_url: str | None = api_url or os.getenv("API_INGEST_URL")
    if not target_url:
        logger.warning("[GLIA_PYTHON] No API_INGEST_URL configured. Telemetry dropped.")
        return False

    final_timeout = timeout if timeout is not None else 2.0
    try:
        # The backend now only accepts batches (list of jobs)
        json_payload = f"[{metrics.model_dump_json()}]"
        core.queue_telemetry(json_payload, target_url, final_timeout)
        return True

    except Exception as e:
        logger.warning(f"[GLIA_PYTHON] Unexpected error during telemetry queuing: {e}")
        return False
