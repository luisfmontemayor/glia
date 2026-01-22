from datetime import UTC, datetime
from unittest.mock import MagicMock, patch

import httpx
from glia_python.JobMetrics import JobMetrics
from glia_python.network import send_telemetry


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


# 1. Glia is able to serialise and send data to db endpoint
@patch("glia_python.network.httpx.post")
def test_send_telemetry_success(mock_post):
    """Verify that metrics are correctly serialized and sent."""
    metrics = create_sample_metrics()
    mock_response = MagicMock()
    mock_response.raise_for_status.return_value = None
    mock_post.return_value = mock_response

    success: bool = send_telemetry(metrics, api_url="http://localhost:8000")

    assert success is True

    mock_post.assert_called_once()
    call_kwargs = mock_post.call_args.kwargs
    payload = call_kwargs["json"]

    assert payload["run_id"] == "test-uuid"
    assert payload["wall_time_sec"] == 10.0
    assert payload["started_at"] == "2025-01-01T12:00:00+00:00"
    assert payload["meta"] == {"env": "test"}


# 2. Db push fails if no URL or env var are given
def test_send_telemetry_no_config():
    metrics = create_sample_metrics()

    # Execute with no URL and no Env Var
    with patch.dict("os.environ", {}, clear=True):
        success = send_telemetry(metrics)

    assert success is False


# 3. Glia reads GLIA_API_URL from environment variable if used
@patch("os.getenv")
@patch("glia_python.network.httpx.post")
def test_send_telemetry_uses_env_var(mock_post, mock_getenv):
    metrics: JobMetrics = create_sample_metrics()
    mock_getenv.return_value = "http://env-var-url:9000/"
    mock_response = MagicMock()
    mock_response.raise_for_status.return_value = None
    mock_post.return_value = mock_response

    success: bool = send_telemetry(metrics)

    assert success is True
    mock_getenv.assert_called_once_with("GLIA_API_URL")
    mock_post.assert_called_once()
    call_args = mock_post.call_args
    assert call_args.args[0] == "http://env-var-url:9000/jobs/"


# 4. Verify that HTTP status errors are caught and do not crash.
@patch("glia_python.network.httpx.post")
def test_send_telemetry_http_error(mock_post):
    metrics: JobMetrics = create_sample_metrics()
    # Simulate a 400 Bad Request
    mock_response = MagicMock()
    mock_response.status_code = 400
    mock_response.text = "Invalid payload"
    mock_response.raise_for_status.side_effect = httpx.HTTPStatusError(
        "Bad Request", request=MagicMock(), response=mock_response
    )
    mock_post.return_value = mock_response

    # Execute (Should not raise exception)
    success: bool = send_telemetry(metrics, api_url="http://some-url")

    assert success is False
    mock_post.assert_called_once()


# 5. Verify that network timeouts are caught and do not crash the app.
@patch("glia_python.network.httpx.post")
def test_send_telemetry_fail_fast(mock_post):
    metrics: JobMetrics = create_sample_metrics()

    mock_post.side_effect = httpx.TimeoutException("Connection timed out")

    success: bool = send_telemetry(metrics, api_url="http://bad-url")

    assert success is False
