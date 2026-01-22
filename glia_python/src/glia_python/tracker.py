import getpass
import hashlib
import os
import platform
import sys
import time
import uuid
from datetime import UTC, datetime
from pathlib import Path
from types import TracebackType
from typing import Any

import psutil

try:
    import resource
except ImportError:
    pass

from glia_python.JobMetrics import JobMetrics
from glia_python.network import send_telemetry


class JobTracker:
    process: psutil.Process
    _start_time: float | None
    _cpu_start: Any | None
    _user_meta: dict[str, Any]
    run_id: str
    user_name: str
    script_path: Path | None
    program_name: str
    script_sha256: str
    metrics: JobMetrics | None

    def __init__(
        self, program_name: str | None = None, context: dict[str, Any] | None = None
    ) -> None:
        self.process = psutil.Process()
        self.metrics = None
        self._start_time = None
        self._cpu_start = None
        self._user_meta = context or {}

        self.run_id = str(uuid.uuid4())
        self.user_name = getpass.getuser()

        if sys.argv[0]:
            self.script_path = Path(os.path.abspath(sys.argv[0]))
            self.script_sha256 = self._calculate_sha256(self.script_path)
            base_name = self.script_path.name
        else:
            self.script_path = None
            self.script_sha256 = "unknown-hash"
            base_name = "interactive"

        if program_name:
            self.program_name = f"{base_name}:{program_name}"
        else:
            self.program_name = base_name

    def start(self) -> None:
        self._start_time = time.time()
        self._cpu_start = self.process.cpu_times()

    def log_metadata(self, data: dict[str, Any]) -> None:
        """
        Add custom values.
        Example: tracker.log_metadata({"dataset": "mnist", "epoch": 10})
        """
        self._user_meta.update(data)

    def __enter__(self) -> "JobTracker":
        self.start()
        return self

    def __exit__(
        self,
        exc_type: type[BaseException] | None,
        exc_val: BaseException | None,
        exc_tb: TracebackType | None,
    ) -> None:
        exit_code = 1 if exc_type else 0
        self.metrics = self.capture(exit_code=exit_code)
        send_telemetry(self.metrics)
        return None

    def _calculate_sha256(self, file_path: Path) -> str:
        sha256: hashlib._Hash = hashlib.sha256()
        try:
            with file_path.open("rb") as f:
                for chunk in iter(lambda: f.read(4096), b""):
                    sha256.update(chunk)
            return sha256.hexdigest()
        except OSError:
            return "access-denied"

    def _get_peak_rss_mb(self) -> float:
        if sys.platform != "win32" and "resource" in sys.modules:
            usage: float = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
            if sys.platform == "darwin":
                return usage / (1024**2)
            return usage / 1024
        return self.process.memory_info().rss / (1024**2)

    def capture(self, exit_code: int = 0) -> JobMetrics:
        if self._start_time is None or self._cpu_start is None:
            raise RuntimeError(
                "JobTracker was not started. Call .start() or use 'with JobTracker():'"
            )

        end_time: float = time.time()
        cpu_end: Any = self.process.cpu_times()

        cpu_total_start: float = self._cpu_start.user + self._cpu_start.system
        cpu_total_end: float = cpu_end.user + cpu_end.system
        cpu_time_consumed: float = cpu_total_end - cpu_total_start

        wall_time: float = end_time - self._start_time

        cpu_percent: float = 0.0
        if wall_time > 0.0001:
            cpu_percent = (cpu_time_consumed / wall_time) * 100

        return JobMetrics(
            run_id=self.run_id,
            program_name=self.program_name,
            user_name=self.user_name,
            script_sha256=self.script_sha256,
            hostname=platform.node(),
            os_info=f"{platform.system()} {platform.release()}",
            script_path=str(self.script_path) if self.script_path else None,
            argv=sys.argv[1:],
            wall_time_sec=wall_time,
            started_at=datetime.fromtimestamp(self._start_time, tz=UTC),
            ended_at=datetime.fromtimestamp(end_time, tz=UTC),
            cpu_time_sec=cpu_time_consumed,
            cpu_percent=round(cpu_percent, 2),
            max_rss_mb=round(self._get_peak_rss_mb(), 2),
            exit_code_int=exit_code,
            meta=self._user_meta,
        )
