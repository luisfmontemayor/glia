from datetime import UTC, datetime
from unittest.mock import MagicMock, patch

import httpx
from glia_python.job_tracker import JobMetrics
from glia_python.network import send_telemetry


# --- Fixtures ---
def create_sample_metrics() -> JobMetrics:
    return JobMetrics(
        run_id="test-uuid",
        program_name="test-prog",
        user_name="test-user",
        script_sha256="hash",
        hostname="localhost",
        os_info="Linux",
        script_path="/tmp/script.py",
        argv=["--arg"],
        started_at=datetime(2025, 1, 1, 12, 0, 0, tzinfo=UTC),
        ended_at=datetime(2025, 1, 1, 12, 0, 10, tzinfo=UTC),
        wall_time_sec=10.0,
        cpu_time_sec=5.0,
        cpu_percent=50.0,
        max_rss_mb=100.0,
        exit_code_int=0,
        meta={"env": "test"},
    )


# --- Tests ---


@patch("glia_python.network.httpx.post")
def test_send_telemetry_success(mock_post):
    """Verify that metrics are correctly serialized and sent."""
    # Setup
    metrics = create_sample_metrics()
    mock_response = MagicMock()
    mock_response.raise_for_status.return_value = None
    mock_post.return_value = mock_response

    # Execute
    success = send_telemetry(metrics, api_url="http://localhost:8000")

    # Assertions
    assert success is True

    # Check what was sent (Payload Builder Verification)
    mock_post.assert_called_once()
    call_kwargs = mock_post.call_args.kwargs
    payload = call_kwargs["json"]

    assert payload["run_id"] == "test-uuid"
    assert payload["wall_time_sec"] == 10.0
    # Verify datetime serialization
    assert payload["started_at"] == "2025-01-01T12:00:00+00:00"
    assert payload["meta"] == {"env": "test"}


@patch("glia_python.network.httpx.post")
def test_send_telemetry_fail_fast(mock_post):
    """Verify that network timeouts are caught and do not crash the app."""
    metrics = create_sample_metrics()

    # Simulate a Timeout
    mock_post.side_effect = httpx.TimeoutException("Connection timed out")

    # Execute (Should not raise exception)
    success = send_telemetry(metrics, api_url="http://bad-url")

    assert success is False


def test_send_telemetry_no_config():
    """Verify behavior when no API URL is provided."""
    metrics = create_sample_metrics()

    # Execute with no URL and no Env Var
    with patch.dict("os.environ", {}, clear=True):
        success = send_telemetry(metrics)

    assert success is False
