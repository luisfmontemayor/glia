import json
from datetime import UTC, datetime
from unittest.mock import MagicMock, patch

from glia_python.JobMetrics import JobMetrics
from glia_python.network import push_telemetry


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


@patch("glia_python.network.glia_core.push_telemetry")
def test_push_telemetry_success(mock_push):
    """Verify that metrics are correctly serialized and sent."""
    metrics = create_sample_metrics()

    mock_result = MagicMock()
    mock_result.status = 201
    mock_result.body = "Created"
    mock_push.return_value = mock_result

    success: bool = push_telemetry(metrics, api_url="http://localhost:8000/ingest")

    assert success is True
    mock_push.assert_called_once()

    json_payload = mock_push.call_args.args[0]
    payload = json.loads(json_payload)
    assert payload["run_id"] == "test-uuid"
    assert payload["wall_time_sec"] == 10.0


@patch("os.getenv")
@patch("glia_python.network.glia_core.push_telemetry")
def test_push_telemetry_uses_env_var(mock_push, mock_getenv):
    metrics = create_sample_metrics()
    mock_getenv.return_value = "http://env-var-url:9000"

    mock_result = MagicMock()
    mock_result.status = 200
    mock_push.return_value = mock_result

    success = push_telemetry(metrics)

    assert success is True
    mock_getenv.assert_called_once_with("API_INGEST_URL")
    assert mock_push.call_args.args[1] == "http://env-var-url:9000"


@patch("glia_python.network.glia_core.push_telemetry")
def test_push_telemetry_http_error(mock_push):
    metrics = create_sample_metrics()

    # Simulate a 400 Bad Request returned by glia_core
    mock_result = MagicMock()
    mock_result.status = 400
    mock_result.body = "Bad Request"
    mock_push.return_value = mock_result

    success = push_telemetry(metrics, api_url="http://some-url")

    assert success is False
    mock_push.assert_called_once()


@patch("glia_python.network.glia_core.push_telemetry")
def test_push_telemetry_fail_fast(mock_push):
    metrics = create_sample_metrics()

    # Simulate an exception inside the core push logic (e.g. connection error)
    mock_push.side_effect = Exception("Connection timed out")

    success = push_telemetry(metrics, api_url="http://bad-url")

    assert success is False
