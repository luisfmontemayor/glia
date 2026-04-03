import argparse
import json
import os
import time

from common.logs import setup_logger
from glia_python.tracker import JobTracker

logger = setup_logger("GLIA_PYTHON STRESS TEST")


def run_benchmark(iterations):
    """
    Performance test for the 'Client-to-Core' inflection point.
    Target: Measure the delta introduced by JobTracker -> core hand-off.
    """
    stress_load = iterations * 5

    logger.info(f"Starting stress test: {iterations} jobs (Load Factor: {stress_load})")

    start_time = time.perf_counter()

    for i in range(iterations):
        with JobTracker(program_name="stress_worker") as tracker:
            tracker.log_metadata({"iteration": i, "stress_load": stress_load})
            pass

    end_time = time.perf_counter()

    total_duration = end_time - start_time
    avg_overhead_ms = (total_duration / iterations) * 1000
    throughput = iterations / total_duration

    report = {
        "metric_type": "client_to_core_python",
        "load": iterations,
        "throughput": round(throughput, 2),
        "latency_ms": round(avg_overhead_ms, 4),
        "success_rate": 1.0,
    }

    print(f"REPORT_START{json.dumps(report)}REPORT_END")

    logger.info("Client-to-Core stress test complete.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run Client-to-Core Python benchmark")
    parser.add_argument(
        "--iterations",
        type=int,
        default=int(os.environ.get("CORE_QUEUE_LIMIT", "1000")),
        help="Number of iterations to run",
    )
    args = parser.parse_args()
    run_benchmark(args.iterations)
