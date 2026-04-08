from datetime import datetime, timezone
from uuid import uuid4

import pytest
import pytest_asyncio
from common.cli import run_command
from httpx import AsyncClient
from sqlmodel import delete

from backend.config import settings
from backend.database import engine
from backend.models import Job


def test_api_status():
    run_command(command=["mise", "run", "api:status"], capture=False)


@pytest_asyncio.fixture
async def ingest_cleanup_client():
    job_ids = [str(uuid4()) for _ in range(3)]
    async with AsyncClient(
        base_url=f"http://{settings.API_HOST}:{settings.API_PORT}",
    ) as client:
        yield client, job_ids

    print(f"\n[CLEANUP] Removing Job IDs: {job_ids}")
    async with engine.begin() as conn:
        await conn.execute(delete(Job).where(Job.run_id.in_(job_ids)))


@pytest.mark.asyncio
async def test_ingest(ingest_cleanup_client):
    client, job_ids = ingest_cleanup_client
    payloads = [
        {
            "run_id": job_id,
            "program_name": f"pytest_runner_{i}.py",
            "user_name": "ci_bot",
            "hostname": "test-host",
            "os_info": "Linux-Test",
            "script_sha256": f"hash_{i}",
            "started_at": datetime.now(timezone.utc).isoformat(),
            "ended_at": datetime.now(timezone.utc).isoformat(),
            "wall_time_ms": 100,
            "exit_code_int": 0,
            "cpu_time_sec": 1.2,
            "cpu_percent": 15.5,
            "max_rss_kb": 262144,
            "meta": {"batch_index": i},
        }
        for i, job_id in enumerate(job_ids)
    ]

    response = await client.post("/ingest", json=payloads)

    assert response.status_code == 201
    data = response.json()
    assert isinstance(data, list)
    assert len(data) == 3
    for i, job in enumerate(data):
        assert job["run_id"] == job_ids[i]
