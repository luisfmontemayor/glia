from datetime import datetime
from typing import Any
from uuid import UUID, uuid4

from sqlalchemy import Column, DateTime
from sqlalchemy.dialects.postgresql import JSONB
from sqlmodel import Field, SQLModel


class JobBase(SQLModel):
    run_id: UUID = Field(default_factory=uuid4, index=True, unique=True)

    hostname: str = Field(index=True, description="The machine where the job ran")
    os_info: str = Field(description="OS name and version")
    user_name: str = Field(index=True)

    script_path: str | None = Field(default=None)
    argv: list[str] = Field(default_factory=list, sa_column=Column(JSONB))
    script_sha256: str = Field()

    program_name: str = Field(
        index=True,
        description="Defaults to function being tracked, can be user defined",
    )
    started_at: datetime = Field(sa_column=Column(DateTime(timezone=True), index=True))
    ended_at: datetime = Field(sa_column=Column(DateTime(timezone=True)))
    wall_time_sec: float = Field()
    cpu_time_sec: float = Field()
    cpu_percent: float = Field()
    max_rss_mb: float = Field()
    exit_code_int: int = Field()

    meta: dict[str, Any] = Field(
        default={},
        sa_column=Column(JSONB),
        description="Strictly for user-defined tags and business logic context",
    )


class Job(JobBase, table=True):
    __tablename__ = "jobs"
    id: int | None = Field(default=None, primary_key=True)


class JobCreate(JobBase):
    pass


class JobRead(JobBase):
    id: int
