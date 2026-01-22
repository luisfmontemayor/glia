# Written by Luis Felipe Montemayor, sometime around January of 2026
import os
from typing import Any

import httpx

from .tracker import JobMetrics


def _to_payload(metrics: JobMetrics) -> dict[str, Any]:
    """
    Payload Builder: Maps JobMetrics to the Backend JSON schema.
    Handles serialization of types like datetime.
    """
    return {
        "run_id": metrics.run_id,
        "program_name": metrics.program_name,
        "user_name": metrics.user_name,
        "script_sha256": metrics.script_sha256,
        # System Metrics
        "hostname": metrics.hostname,
        "os_info": metrics.os_info,
        "script_path": metrics.script_path,
        "argv": metrics.argv,
        # Performance
        "started_at": metrics.started_at.isoformat(),
        "ended_at": metrics.ended_at.isoformat(),
        "wall_time_sec": metrics.wall_time_sec,
        "cpu_time_sec": metrics.cpu_time_sec,
        "cpu_percent": metrics.cpu_percent,
        "max_rss_mb": metrics.max_rss_mb,
        "exit_code_int": metrics.exit_code_int,
        # User Metadata
        "meta": metrics.meta,
    }


def send_telemetry(
    metrics: JobMetrics, api_url: str | None = None, timeout: float = 2.0
) -> bool:
    """
    Push telemetry to the backend.

    Features:
    - Auto-config: Reads GLIA_API_URL env var if api_url is not provided.
    - Fail Fast: Short timeout (default 2s) to avoid hanging the main job.
    - Suppress Errors: Returns False on failure instead of crashing.
    """
    # 1. Configuration
    target_url = api_url or os.getenv("GLIA_API_URL")
    if not target_url:
        # If no URL is configured, we simply skip silently (or log debug)
        return False

    endpoint = f"{target_url.rstrip('/')}/jobs/"
    payload = _to_payload(metrics)

    try:
        # 2. Send Request
        response = httpx.post(endpoint, json=payload, timeout=timeout)
        response.raise_for_status()
        return True

    except httpx.HTTPStatusError as e:
        # 3. Error Handling (Log but don't crash)
        print(
            f"[Glia] Warning: Failed to push telemetry. Status: {e.response.status_code}"
        )
        print(f"[Glia] Detail: {e.response.text}")
        return False

    except (httpx.ConnectError, httpx.TimeoutException, httpx.RequestError):
        # We suppress connection errors entirely to avoid disrupting user workflows
        print("[Glia] Warning: Could not connect to observability backend.")
        return False
