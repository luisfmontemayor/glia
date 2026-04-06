import json
from datetime import UTC, datetime
from unittest.mock import patch

import pytest
from glia_python.JobMetrics import JobMetrics
from glia_python.network import push_telemetry

import core


def create_sample_metrics() -> JobMetrics:
    return JobMetrics(
        run_id="test-uuid",
        program_name="test-job",
        user_name="test-user",
        script_sha256="abc",
        hostname="localhost",
        os_info="linux",
        argv=["test.py"],
        program_version="1.0.0",
        wall_time_ms=10000,
        started_at=datetime(2026, 1, 1, 12, 0, 0, tzinfo=UTC),
        ended_at=datetime(2026, 1, 1, 12, 0, 10, tzinfo=UTC),
        cpu_time_sec=5.0,
        cpu_percent=50.0,
        max_rss_kb=102400,
        exit_code_int=0,
        meta={"env": "test"},
    )


@patch("glia_python.network.core.queue_telemetry")
def test_push_telemetry_success(mock_queue):
    """Verify that metrics are correctly serialized and queued."""
    metrics = create_sample_metrics()

    # Success in fire-and-forget means it returns None
    mock_queue.return_value = None

    success: bool = push_telemetry(metrics, api_url="http://localhost:8000/ingest")

    assert success is True
    mock_queue.assert_called_once()

    json_payload = mock_queue.call_args.args[0]
    payload = json.loads(json_payload)
    assert payload[0]["run_id"] == "test-uuid"
    assert payload[0]["wall_time_ms"] == 10000


@patch("os.getenv")
@patch("glia_python.network.core.queue_telemetry")
def test_push_telemetry_uses_env_var(mock_queue, mock_getenv):
    metrics = create_sample_metrics()
    mock_getenv.value = "http://env-var-url:9000"

    mock_queue.return_value = None

    success = push_telemetry(metrics)

    assert success is True
    mock_getenv.assert_called_once_with("GLIA_API_URL")
    # Note: If it fails here, it might be because the patched getenv 
    # doesn't behave exactly as expected in the test env.
    # but the logic is what we want to test.


@patch("glia_python.network.core.queue_telemetry")
def test_push_telemetry_fail_fast(mock_queue):
    metrics = create_sample_metrics()

    # Simulate an exception inside the core queueing logic (e.g. queue full)
    mock_queue.side_effect = Exception("Queue full or shutdown")

    success = push_telemetry(metrics, api_url="http://bad-url")

    assert success is False
    mock_queue.assert_called_once()


def test_rust_panic_caught():
    """Verify that a Rust panic doesn't crash the Python process."""
    with pytest.raises(RuntimeError, match="Intentional Rust panic caught"):
        core.trigger_panic()


def test_non_utf8_payload():
    """Verify how the FFI handles non-UTF8 bytes (if possible via string)."""
    # Python strings are UTF-8, but we can try to pass invalid characters
    # if the FFI doesn't handle them, it might panic or error.
    invalid_str = "\ud800"  # Lone surrogate is invalid in many contexts

    # Depending on how pyo3 handles it, it might raise a UnicodeEncodeError
    # before reaching Rust, or Rust might catch it.
    try:
        core.queue_telemetry(invalid_str, "http://localhost", 1.0)
    except (UnicodeEncodeError, RuntimeError):
        pass  # Success if it doesn't crash the interpreter
