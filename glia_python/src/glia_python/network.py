# Written by Luis Felipe Montemayor, sometime around January of 2026
import os

from glia_common.logs import setup_logger

import glia_core
from glia_core import PushResult
from glia_python.JobMetrics import JobMetrics

logger = setup_logger("glia")


def push_telemetry(
    metrics: JobMetrics, api_url: str | None = None, timeout: float = 2.0
) -> bool:
    """
    - Auto-config: Reads GLIA_API_URL env var if api_url is not provided.
    - Suppress Errors: Returns False on failure instead of crashing (Fail Silent).
    """
    target_url: str | None = api_url or os.getenv("GLIA_API_URL")
    if not target_url:
        logger.warning("[Glia] No GLIA_API_URL configured. Telemetry dropped.")
        return False

    try:
        json_payload = metrics.model_dump_json()
        result: PushResult = glia_core.push_telemetry(json_payload, target_url, timeout)

        if 200 <= result.status < 300:
            return True

        logger.warning(f"[Glia] Backend returned error {result.status}: {result.body}")
        return False

    except ConnectionError as e:
        logger.warning(f"[Glia] Could not connect to backend: {e}")
        return False

    except Exception as e:
        logger.warning(f"[Glia] Unexpected error during push: {e}")
        return False
