import json
import uuid
from unittest.mock import MagicMock, patch

from fastapi.testclient import TestClient

from backend.main import app
from glia_python import Glia


def test_end_to_end_telemetry_flow(monkeypatch):
    """
    Simulates a full run:
    1. Synchronous environment for the synchronous Glia client.
    2. Unique ID to prevent stale data collisions.
    3. Correct API_INGEST_URL environment variable.
    """
    with TestClient(app) as server:
        # 1. SETUP:
        unique_id = uuid.uuid4().hex[:6]
        unique_name = f"e2e_{unique_id}"
        monkeypatch.setenv("API_INGEST_URL", "http://localhost:8000/ingest")

        # 2. MOCK: Must be a regular 'def' (synchronous)
        def mock_glia_core_push(json_payload, target_url, timeout):
            path = "/" + target_url.split("/", 3)[-1]

            response = server.post(path, json=json.loads(json_payload))

            mock_result = MagicMock()
            mock_result.status = response.status_code
            mock_result.body = response.text
            return mock_result

        # 3. EXECUTE: Patch and run the tracker
        with patch(
            "glia_python.network.glia_core.push_telemetry",
            side_effect=mock_glia_core_push,
        ):
            with Glia.tracker(program_name=unique_name, context={"e2e": "true"}):
                _x = 1 + 1

        # 4. VERIFY: Query the list endpoint
        response = server.get("/telemetry")
        assert response.status_code == 200

        jobs = response.json()
        assert len(jobs) > 0

        # Get the job we just created (filtering by our unique ID)
        current_job = next(
            (j for j in reversed(jobs) if unique_id in j["program_name"]), None
        )

        assert current_job is not None, f"Job with {unique_id} not found in DB"
        # JobTracker prefixes script name (e.g. 'pytest:e2e_abc123')
        assert f"pytest:{unique_name}" in current_job["program_name"]
        assert current_job["meta"]["e2e"] == "true"

    print(f"\n✅ E2E Success: Verified job {unique_name}")
