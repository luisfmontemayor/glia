import os
import time

import pytest
from glia_common.logs import setup_logger
from glia_python.tracker import JobTracker

logger = setup_logger("GLIA_PYTHON STRESS TEST")

ITERATIONS = 1000
# Using .get() to prevent KeyError during pytest collection if env is missing
STRESS_LOAD = int(os.environ.get("GLIA_CORE_QUEUE_LIMIT", "1000"))


@pytest.fixture
def stress_config():
    """Provides configuration for stress test inflection points."""
    return {
        "load_factor": STRESS_LOAD * 5,
        "iterations": ITERATIONS,
        "target_url": os.environ.get("API_INGEST_URL", "http://localhost:8000/ingest"),
    }


@pytest.mark.performance
@pytest.mark.skipif(
    os.getenv("GLIA_PERFORMANCE_TEST") != "true",
    reason="GLIA_PERFORMANCE_TEST environment variable is not set to 'true'",
)
def test_client_to_core_overhead(stress_config):
    """
    Performance test for the 'Client-to-Core'
    Target: Measure the delta introduced by JobTracker -> glia_core hand-off.
    """
    iterations = stress_config["iterations"]
    stress_load = stress_config["load_factor"]

    logger.info(f"Starting stress test: {iterations} jobs (Load Factor: {stress_load})")

    start_time = time.perf_counter()

    for i in range(iterations):
        # Tracker context manager triggers capture and push logic
        with JobTracker(program_name="stress_worker") as tracker:
            tracker.log_metadata({"iteration": i, "stress_load": stress_load})
            pass

    end_time = time.perf_counter()

    total_duration = end_time - start_time
    avg_overhead_ms = (total_duration / iterations) * 1000
    throughput = iterations / total_duration

    logger.info("Client-to-Core stress test complete.")
    logger.info(f"Duration: {total_duration:.2f}s")
    logger.info(f"Avg. Glia overhead: {avg_overhead_ms:.4f}ms")
    logger.info(f"Measured Throughput: {throughput:.2f} jobs/sec")

    # Technical Enemy: Observer effect must be minimal
    assert avg_overhead_ms < 5.0, f"Observer effect too high: {avg_overhead_ms:.4f}ms"
