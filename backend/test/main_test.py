from datetime import UTC, datetime
from uuid import uuid4

import pytest
import pytest_asyncio
from httpx import ASGITransport, AsyncClient
from sqlmodel import text

from backend.database import engine
from backend.main import app


@pytest_asyncio.fixture
async def cleanup_client():
    job_id = str(uuid4())
    async with AsyncClient(
        transport=ASGITransport(app=app),
        base_url="http://test",  # URL set up by httpx for testing through ASGITransport
    ) as client:
        yield client, job_id

    print(f"\n[CLEANUP] Removing Job ID: {job_id}")
    async with engine.begin() as conn:
        await conn.execute(text("DELETE FROM jobs WHERE run_id = :id"), {"id": job_id})


@pytest.mark.asyncio
async def test_ingest_telemetry(cleanup_client):
    client, job_id = cleanup_client

    payload = {
        "run_id": job_id,
        "program_name": "pytest_runner.py",
        "user_name": "ci_bot",
        "script_sha256": "uncracked_hash_123",
        "started_at": datetime.now(UTC).isoformat(),
        "ended_at": datetime.now(UTC).isoformat(),
        "exit_code_int": 0,
        "cpu_time_sec": 1.2,
        "cpu_percent": 15.5,
        "max_rss_mb": 256.0,
        "meta": {"region": "eu-west-2", "test_run": True},
    }

    response = await client.post("/ingest", json=payload)

    if response.status_code != 201:
        print(f"API Error Response: {response.text}")

    assert response.status_code == 201

    data = response.json()
    assert data["program_name"] == "pytest_runner.py"
    print(f"[SUCCESS] Ingested and verified Job ID: {job_id}")
