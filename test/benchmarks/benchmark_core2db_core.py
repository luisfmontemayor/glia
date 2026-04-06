import argparse
import json
import os
import time
import uuid
from datetime import UTC, datetime

from common.logs import setup_logger

import core

logger = setup_logger("benchmark_core2db_core.py")

API_URL = os.environ.get("GLIA_API_URL", "http://localhost:8000/ingest")


def push_telemetry_core(url, iteration, load_factor):
    """Uses core.queue_telemetry to hit the FastAPI /ingest endpoint."""
    payload = {
        "run_id": str(uuid.uuid4()),
        "hostname": "db-stress-node-core",
        "os_info": "Linux Performance-Test-Core",
        "user_name": "stress_user",
        "program_name": "db_stress_worker_core",
        "started_at": datetime.now(UTC).isoformat(),
        "ended_at": datetime.now(UTC).isoformat(),
        "wall_time_sec": 0.1,
        "cpu_time_sec": 0.05,
        "cpu_percent": 10.0,
        "max_rss_mb": 50.0,
        "exit_code_int": 0,
        "argv": ["--db-stress-core"],
        "script_sha256": "db-stress-hash-core",
        "meta": {"iteration": iteration, "load": load_factor},
    }

    try:
        # queue_telemetry is synchronous and adds to an internal Rust queue
        core.queue_telemetry(json.dumps(payload), url)
        return True
    except Exception as e:
        logger.error(f"Error queueing telemetry: {e}")
        return False


def run_benchmark(iterations):
    """
    Performance test for the 'Persistence Barrier' using core.
    Measures the time it takes to queue and then FLUSH all telemetry.
    """
    logger.info(f"Starting DB stress test (via core): {iterations} writes")

    # Set the queue limit via environment variable for the Rust core
    os.environ["CORE_QUEUE_LIMIT"] = str(iterations + 100)

    start_time = time.perf_counter()
    
    success_count = 0
    for i in range(iterations):
        if push_telemetry_core(API_URL, i, iterations):
            success_count += 1
            
    # The actual "DB write" latency in this model is mostly in the FLUSH
    # because queue_telemetry is just a memory push.
    logger.info(f"Queued {success_count} items. Flushing to DB...")
    
    flush_start = time.perf_counter()
    summary = core.flush_queue()
    flush_end = time.perf_counter()
    
    total_duration = flush_end - start_time
    flush_duration_ms = (flush_end - flush_start) * 1000
    
    # In this model, we consider the "latency" as the average time per job 
    # from the moment we started queueing until everything is flushed.
    avg_latency_ms = (total_duration / iterations) * 1000 if iterations > 0 else 0
    
    success_rate = (iterations - summary.failed_jobs) / iterations if iterations > 0 else 0
    throughput = iterations / total_duration if total_duration > 0 else 0

    report = {
        "metric_type": "backend_to_db_core",
        "load": iterations,
        "throughput": round(throughput, 2),
        "latency_ms": round(avg_latency_ms, 2),
        "flush_duration_ms": round(flush_duration_ms, 2),
        "success_rate": round(success_rate, 2),
        "failed_jobs": summary.failed_jobs
    }

    print(f"REPORT_START{json.dumps(report)}REPORT_END")

    logger.info(f"Backend-to-DB (core) stress test complete. Failed: {summary.failed_jobs}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run Backend-to-DB benchmark using core")
    parser.add_argument(
        "--iterations",
        type=int,
        default=int(os.environ.get("CORE_QUEUE_LIMIT", "1000")),
        help="Number of iterations to run",
    )
    args = parser.parse_args()
    run_benchmark(args.iterations)
