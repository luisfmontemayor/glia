# Written by Luis Felipe Montemayor, sometime around January of 2026
from datetime import datetime
from typing import Any

from pydantic import BaseModel, ConfigDict, Field


class JobMetrics(BaseModel):
    model_config = ConfigDict(strict=True)

    run_id: str
    program_name: str
    user_name: str
    script_sha256: str
    hostname: str
    os_info: str
    script_path: str | None = None
    argv: list[str]
    wall_time_sec: float
    started_at: datetime
    ended_at: datetime
    cpu_time_sec: float
    cpu_percent: float
    max_rss_mb: float
    exit_code_int: int
    meta: dict[str, Any] = Field(default_factory=dict)
