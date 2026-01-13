import hashlib
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

    # 1. CPU Times
    mock_process.cpu_times.side_effect = [
        MagicMock(user=0.0, system=0.0),  # Start (called in .start())
        MagicMock(user=8.0, system=2.0),  # End (called in .capture())
    ]

    # 2. Wall Time
    mock_time.time.side_effect = [
        1000.0,
        1020.0,
    ]

    return mock_process


def setup_platform_ram(mock_sys, mock_resource, platform: str, usage_val: int):
    """
    Configures sys and resource modules to simulate a specific OS RAM usage.
    """
    mock_sys.platform = platform
    mock_sys.modules = {"resource": mock_resource}
    mock_resource.getrusage.return_value.ru_maxrss = usage_val


# 1. The tracker correctly captures CPU and Wall time when used as a context manager.
@patch("glia_python.job_tracker.psutil")
@patch("glia_python.job_tracker.time")
def test_tracker_happy_path(mock_time, mock_psutil):
    setup_system_mocks(mock_psutil, mock_time)

    with Glia.tracker(program_name="test_job") as tracker:
        pass

    assert tracker.metrics is not None
    assert tracker.metrics.cpu_time_sec == 10.0
    assert tracker.metrics.program_name == "test_job"
    assert tracker.metrics.cpu_percent == 50.0


# 2. The @Glia.track decorator correctly initializes and runs the JobTracker.
@patch("glia_python.job_tracker.psutil")
@patch("glia_python.job_tracker.time")
def test_decorator_usage(mock_time, mock_psutil):
    """
    Verify the decorator fully integrates with JobTracker logic by
    simulating a run and inspecting the captured metrics on the
    ephemeral tracker instance.
    """
    setup_system_mocks(mock_psutil, mock_time)

    # --- Spy on JobTracker ---
    captured_trackers = []
    real_JobTracker = JobTracker

    def tracker_spy(*args, **kwargs):
        instance = real_JobTracker(*args, **kwargs)
        captured_trackers.append(instance)
        return instance

    # Patch JobTracker where it is used (in glia_python.__init__)
    with patch("glia_python.JobTracker", side_effect=tracker_spy):

        @Glia.track(program_name="decorated_func")
        def my_function():
            return "success"

        result = my_function()

    assert result == "success"
    assert len(captured_trackers) == 1

    tracker = captured_trackers[0]
    assert tracker.metrics is not None
    assert tracker.metrics.program_name == "decorated_func"
    assert tracker.metrics.cpu_time_sec == 10.0
    assert tracker.metrics.cpu_percent == 50.0


# 3. The tracker captures the exit code '1' when an exception is raised within the context manager.
def test_context_manager_exception():
    with pytest.raises(ValueError):
        with Glia.tracker() as tracker:
            raise ValueError("Crash!")

    assert tracker.metrics is not None
    assert tracker.metrics.exit_code_int == 1


# 4. The tracker correctly normalizes Linux memory usage (KB) to MB.
@patch("glia_python.job_tracker.sys")
@patch("glia_python.job_tracker.resource")
def test_linux_ram_conversion(mock_resource, mock_sys):
    # Linux returns KB (102,400 KB = 100 MB)
    setup_platform_ram(mock_sys, mock_resource, "linux", 102_400)

    tracker = JobTracker()
    assert tracker._get_peak_rss_mb() == 100.0


# 5. The tracker correctly normalizes macOS memory usage (Bytes) to MB.
@patch("glia_python.job_tracker.sys")
@patch("glia_python.job_tracker.resource")
def test_mac_ram_conversion(mock_resource, mock_sys):
    # macOS returns Bytes (104,857,600 B = 100 MB)
    setup_platform_ram(mock_sys, mock_resource, "darwin", 104_857_600)

    tracker = JobTracker()
    assert tracker._get_peak_rss_mb() == 100.0


# 6. The tracker correctly normalizes Windows memory usage (Bytes from psutil) to MB.
@patch("glia_python.job_tracker.sys")
@patch("glia_python.job_tracker.psutil")
def test_windows_ram_conversion(mock_psutil, mock_sys):
    mock_sys.platform = "win32"
    mock_sys.modules = {}  # Windows has no resource module

    mock_process = MagicMock()
    mock_psutil.Process.return_value = mock_process
    # Windows/psutil returns Bytes (52,428,800 B = 50 MB)
    mock_process.memory_info.return_value.rss = 52_428_800

    tracker = JobTracker()
    assert tracker._get_peak_rss_mb() == 50.0


# 7. The tracker accurately calculates the SHA256 hash of a given script file.
def test_sha256_calculation_correctness():
    content = b"hello"
    expected_sha = hashlib.sha256(content).hexdigest()

    with patch("pathlib.Path.open", mock_open(read_data=content)):
        tracker = JobTracker()
        sha = tracker._calculate_sha256(Path("dummy_script.py"))
        assert sha == expected_sha


# 8. The tracker handles file permission errors gracefully during SHA calculation.
def test_sha256_access_denied():
    with patch("pathlib.Path.open", side_effect=OSError("Permission denied")):
        tracker = JobTracker()
        sha = tracker._calculate_sha256(Path("secret.py"))
        assert sha == "access-denied"
