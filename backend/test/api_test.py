from uuid import uuid4

import httpx
import pytest

API_URL = "http://localhost:8000"


@pytest.mark.asyncio
async def test_ingest_telemetry():
    # Define an example payload
    payload = {
        "job_id": str(uuid4()),
        "client_type": "python-client",
        "cpu_usage": 15.5,
        "memory_usage_mb": 256.0,
        "io_read_mb": 10.2,
        "io_write_mb": 5.1,
        "metadata": {"user": "test_runner", "region": "eu-west-2"},
    }

    async with httpx.AsyncClient(base_url=API_URL) as client:
        response = await client.post("/ingest", json=payload)

    assert response.status_code == 200, f"Failed: {response.text}"
    data = response.json()
    assert data["status"] == "ok"  # Or whatever your API returns
    print(f"\n[SUCCESS] Ingested Job ID: {payload['job_id']}")
