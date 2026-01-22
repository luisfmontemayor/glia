# Written by Luis Felipe Montemayor, sometime around January of 2026
# https://open.spotify.com/track/2wBrtYgA27EDPE7Fpyzpdx?si=6f91ac4ec4594ed8
import hashlib
import re
from pathlib import Path
from unittest.mock import MagicMock, mock_open, patch

import pytest

from glia_python import Glia, JobTracker


def setup_system_mocks(mock_psutil, mock_time):
    """
    Configures mocks for a standard run:
    - CPU Time: 0s start -> 10s end (8s user + 2s system)
    - Wall Time: 1000s start -> 1020s end (20s elapsed)
    Result: 50% CPU load.
    """
    mock_process = MagicMock()
    mock_psutil.Process.return_value = mock_process

    mock_process.cpu_times.side_effect = [
        MagicMock(user=0.0, system=0.0),  # Start
        MagicMock(user=8.0, system=2.0),  # End
    ]

    mock_time.time.side_effect = [
        1000.0,
        1020.0,
    ]

    return mock_process


def setup_platform_ram(mock_sys, mock_resource, platform: str, usage_val: int):
    mock_sys.platform = platform
    mock_sys.modules = {"resource": mock_resource}
    mock_resource.getrusage.return_value.ru_maxrss = usage_val


# 1. The tracker correctly captures CPU/Wall time and infers script name from mocked argv.
@patch("glia_python.tracker.sys")
@patch("glia_python.tracker.psutil")
@patch("glia_python.tracker.time")
def test_context_manager_tracker(mock_time, mock_psutil, mock_sys):
    setup_system_mocks(mock_psutil, mock_time)

    mock_sys.argv = ["/path/to/script.py", "--arg"]

    with Glia.tracker(program_name="test_job") as tracker:
        pass

    assert tracker.metrics is not None
    assert tracker.metrics.cpu_time_sec == 10.0
    assert tracker.metrics.program_name == "script.py:test_job"
    assert tracker.metrics.argv == ["--arg"]


# 2. The @Glia.track decorator correctly initializes and runs.
@patch("glia_python.tracker.sys")
@patch("glia_python.tracker.psutil")
@patch("glia_python.tracker.time")
def test_decorator_usage(mock_time, mock_psutil, mock_sys):
    setup_system_mocks(mock_psutil, mock_time)

    mock_sys.argv = ["app.py"]

    captured_trackers = []
    real_JobTracker = JobTracker

    def tracker_spy(*args, **kwargs):
        instance = real_JobTracker(*args, **kwargs)
        captured_trackers.append(instance)
        return instance

    with patch("glia_python.JobTracker", side_effect=tracker_spy):

        @Glia.track(program_name="decorated_func")
        def my_function():
            return "success"

        result = my_function()

    assert result == "success"
    tracker = captured_trackers[0]

    assert tracker.metrics.program_name == "app.py:decorated_func"


# 3. Test various script detection scenarios.
@patch("glia_python.tracker.sys")
def test_program_name_logic(mock_sys):
    # Case 1: Standard script execution
    mock_sys.argv = ["/abs/path/model_train.py", "--epochs=10"]
    with patch("pathlib.Path.open", mock_open(read_data=b"")):
        t1 = JobTracker(program_name="main")
    assert t1.program_name == "model_train.py:main"

    # Case 2: Interactive session (e.g., Jupyter, Python REPL, etc.)
    mock_sys.argv = [""]
    t2 = JobTracker(program_name="cell_1")
    assert t2.program_name == "interactive:cell_1"
    assert t2.script_sha256 == "unknown-hash"

    # Case 3: No program_name provided
    mock_sys.argv = ["/usr/bin/pytest"]
    with patch("pathlib.Path.open", mock_open(read_data=b"")):
        t3 = JobTracker()
    assert t3.program_name == "pytest"


# 4. The JobMetrics schema correctly separates system telemetry from user metadata.
@patch("glia_python.tracker.platform")
@patch("glia_python.tracker.sys")
def test_metrics_schema_separation(mock_sys, mock_platform):
    mock_platform.node.return_value = "test-host"
    mock_platform.system.return_value = "TestOS"
    mock_platform.release.return_value = "1.0.0"
    mock_sys.argv = ["script.py", "--flag"]

    tracker = JobTracker()
    tracker.start()
    metrics = tracker.capture()

    assert metrics.hostname == "test-host"
    assert metrics.os_info == "TestOS 1.0.0"
    assert metrics.argv == ["--flag"]
    assert "hostname" not in metrics.meta


# 5. The tracker correctly initializes with custom user context.
def test_tracker_initial_context():
    initial_data = {"env": "staging"}
    with JobTracker(context=initial_data) as tracker:
        pass
    assert tracker.metrics.meta["env"] == "staging"


# 6. The log_metadata method correctly merges new data.
def test_log_metadata_merging():
    tracker = JobTracker(context={"tag": "init"})
    tracker.start()
    tracker.log_metadata({"status": "running"})
    metrics = tracker.capture()
    assert metrics.meta["tag"] == "init"
    assert metrics.meta["status"] == "running"


# 7. Exception handling in context manager
def test_context_manager_exception():
    with pytest.raises(ValueError):
        with Glia.tracker() as tracker:
            raise ValueError("Crash!")
    assert tracker.metrics.exit_code_int == 1


# 8. Exception handling in decorator
def test_capture_before_start_raises_error():
    """Verify that calling capture() before start() raises a RuntimeError."""
    tracker = JobTracker()
    with pytest.raises(
        RuntimeError,
        match=re.escape(
            "JobTracker was not started. Call .start() or use 'with JobTracker():'"
        ),
    ):
        tracker.capture()


# 9. RAM Conversion (Linux)
@patch("glia_python.tracker.sys")
@patch("glia_python.tracker.resource")
def test_linux_ram_conversion(mock_resource, mock_sys):
    setup_platform_ram(mock_sys, mock_resource, "linux", 102_400)
    tracker = JobTracker()
    assert tracker._get_peak_rss_mb() == 100.0


# 10. RAM Conversion (macOS)
@patch("glia_python.tracker.sys")
@patch("glia_python.tracker.resource")
def test_mac_ram_conversion(mock_resource, mock_sys):
    setup_platform_ram(mock_sys, mock_resource, "darwin", 104_857_600)
    tracker = JobTracker()
    assert tracker._get_peak_rss_mb() == 100.0


# 11. RAM Conversion (Windows)
@patch("glia_python.tracker.sys")
@patch("glia_python.tracker.psutil")
def test_windows_ram_conversion(mock_psutil, mock_sys):
    mock_sys.platform = "win32"
    mock_sys.modules = {}
    mock_psutil.Process.return_value.memory_info.return_value.rss = 52_428_800
    tracker = JobTracker()
    assert tracker._get_peak_rss_mb() == 50.0


# 12. SHA256 Calculation
def test_sha256_calculation_correctness():
    content = b"hello"
    expected_sha = hashlib.sha256(content).hexdigest()
    with patch("pathlib.Path.open", mock_open(read_data=content)):
        tracker = JobTracker()
        sha = tracker._calculate_sha256(Path("dummy_script.py"))
        assert sha == expected_sha


# 13. SHA256 Permission Error
def test_sha256_access_denied():
    with patch("pathlib.Path.open", side_effect=OSError("Permission denied")):
        tracker = JobTracker()
        sha = tracker._calculate_sha256(Path("secret.py"))
        assert sha == "access-denied"
