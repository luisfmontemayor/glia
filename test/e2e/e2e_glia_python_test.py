import uuid

import httpx
import pytest

from backend.config import settings
from glia_python import Glia


@pytest.mark.asyncio
async def test_end_to_end_telemetry_flow(monkeypatch):
    """
    Simulates a full run against the REAL background API.
    """
    unique_id = uuid.uuid4().hex[:6]
    unique_name = f"e2e_{unique_id}"
    api_url = f"http://{settings.API_HOST}:{settings.API_PORT}/ingest"
    monkeypatch.setenv("GLIA_API_URL", api_url)

    # We don't need to mock core.enqueue_to_background anymore if we want a true E2E
    # But since core is a background worker, we need to ensure it flushes.
    
    with Glia.tracker(program_name=unique_name, context={"e2e": "true"}):
        _x = 1 + 1
    
    # Glia.tracker doesn't have an explicit flush, but glia_python.network uses core
    import core
    core.flush_queue()

    async with httpx.AsyncClient(base_url=f"http://{settings.API_HOST}:{settings.API_PORT}") as client:
        response = await client.get("/telemetry")
        assert response.status_code == 200

        jobs = response.json()
        assert len(jobs) > 0

        current_job = next(
            (j for j in reversed(jobs) if unique_id in j["program_name"]), None
        )

        assert current_job is not None, f"Job with {unique_id} not found in DB. Jobs: {[j['program_name'] for j in jobs[-5:]]}"
        assert f"pytest:{unique_name}" in current_job["program_name"]
        assert current_job["meta"]["e2e"] == "true"

    print(f"\n[SUCCESS] E2E: Verified job {unique_name}")
