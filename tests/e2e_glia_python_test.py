import sys
from pathlib import Path
from unittest.mock import patch

from fastapi.testclient import TestClient

root_dir = Path(__file__).parent.parent
sys.path.append(str(root_dir / "backend"))
sys.path.append(str(root_dir / "src"))


from backend.main import app
from glia_python import Glia


def test_end_to_end_telemetry_flow():
    """
    Simulates a full run:
    1. User runs code with Glia Client.
    2. Client sends data (intercepted).
    3. Backend receives data and saves to DB.
    4. We query the DB to verify integrity.
    """
    server = TestClient(app)

    def mock_post(url, json, timeout):
        path = "/" + url.split("/", 3)[-1]
        return server.post(path, json=json)

    with patch("glia_python.network.httpx.post", side_effect=mock_post):
        with Glia.tracker(program_name="e2e_job", context={"e2e": "true"}) as tracker:
            x = 1 + 1

    response = server.get("/jobs/")
    assert response.status_code == 200

    jobs = response.json()
    assert len(jobs) > 0

    latest_job = jobs[-1]
    assert latest_job["program_name"] == "e2e_job"
    assert latest_job["meta"]["e2e"] == "true"

    print("\n✅ E2E Success: Client data successfully reached the Backend DB!")
