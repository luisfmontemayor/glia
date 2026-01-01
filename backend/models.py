from datetime import datetime
from typing import Any
from uuid import UUID, uuid4

from sqlalchemy import Column
from sqlalchemy.dialects.postgresql import JSONB
from sqlmodel import Field, SQLModel


class JobBase(SQLModel):
    run_id: UUID = Field(
        default_factory=uuid4,
        index=True,
        unique=True,
        description="Unique ID for this specific execution",
    )
    program_name: str = Field(
        index=True, description="Name of the script or application"
    )
    user_name: str = Field(description="System user who ran the job")
    script_sha256: str = Field(
        description="Hash of the script file for version tracking"
    )

    started_at: datetime = Field(description="UTC timestamp when the job started")
    ended_at: datetime = Field(description="UTC timestamp when the job finished")
    exit_code_int: int = Field(description="Process exit code")

    cpu_time_sec: float = Field(description="Total User + System CPU time consumed")
    cpu_percent: float = Field(description="Average CPU usage percentage")
    max_rss_mb: float = Field(description="Maximum Resident Set Size (RAM) in MB")

    meta: dict[str, Any] = Field(
        default={},
        sa_column=Column(JSONB),
        description="Custom fields that can be defined on a per-script basis.",
    )


class Job(JobBase, table=True):
    __tablename__ = "jobs"
    id: int | None = Field(default=None, primary_key=True)
