from dataclasses import dataclass, field
from datetime import datetime
from typing import Any


@dataclass
class JobMetrics:
    run_id: str
    program_name: str
    user_name: str
    script_sha256: str
    hostname: str
    os_info: str
    script_path: str | None
    argv: list[str]
    wall_time_sec: float
    started_at: datetime
    ended_at: datetime
    cpu_time_sec: float
    cpu_percent: float
    max_rss_mb: float
    exit_code_int: int
    meta: dict[str, Any] = field(default_factory=dict)

    def __str__(self) -> str:
        start_str = self.started_at.strftime("%Y-%m-%d %H:%M:%S UTC")
        end_str = self.ended_at.strftime("%H:%M:%S UTC")
        argv_str = " ".join(self.argv)
        return (
            f"\n--- Glia Telemetry ---\n"
            f"Run ID:     {self.run_id}\n"
            f"User:       {self.user_name} on {self.hostname}\n"
            f"OS:         {self.os_info}\n"
            f"Program:    {self.program_name} ({self.script_sha256[:8]}...)\n"
            f"Arguments:  {argv_str}\n"
            f"Time:       {start_str} -> {end_str}\n"
            f"Wall Time:  {self.wall_time_sec:.4f} sec\n"
            f"CPU Time:   {self.cpu_time_sec:.4f} sec ({self.cpu_percent}% avg)\n"
            f"Peak RAM:   {self.max_rss_mb:.2f} MB\n"
            f"Exit Code:  {self.exit_code_int}\n"
            f"User-defined Metadata:    {self.meta}"
        )
