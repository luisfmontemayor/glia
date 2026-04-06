import argparse
import asyncio
import json
import os
import time
import uuid
from datetime import UTC, datetime

import httpx
from common.logs import setup_logger

logger = setup_logger("GLIA_BACKEND STRESS TEST")

API_URL = os.environ.get("GLIA_API_URL", "http://localhost:8000/ingest")


async def push_telemetry_direct(client, url, iteration, load_factor):
    """Simulates a direct hit to the FastAPI /ingest endpoint."""
    payload = {
        "run_id": str(uuid.uuid4()),
        "hostname": "db-stress-node",
        "os_info": "Linux Performance-Test",
        "user_name": "stressed_user",
        "program_name": "db_stress_worker",
        "started_at": datetime.now(UTC).isoformat(),
        "ended_at": datetime.now(UTC).isoformat(),
        "wall_time_sec": 0.1,
        "cpu_time_sec": 0.05,
        "cpu_percent": 10.0,
        "max_rss_mb": 50.0,
        "exit_code_int": 0,
        "argv": ["--db-stress"],
        "script_sha256": "db-stress-hash",
        "meta": {"iteration": iteration, "load": load_factor},
    }

    try:
        start = time.perf_counter()
        response = await client.post(url, json=payload)
        return response.status_code, time.perf_counter() - start
    except Exception:
        return 500, 0


async def run_benchmark(iterations, concurrency):
    """
    Performance test for the 'Persistence Barrier'.
    """
    logger.info(f"Starting DB stress test: {iterations} direct writes")

    limits = httpx.Limits(max_connections=concurrency)
    async with httpx.AsyncClient(limits=limits, timeout=10.0) as client:
        start_time = time.perf_counter()
        tasks = [
            push_telemetry_direct(client, API_URL, i, iterations)
            for i in range(iterations)
        ]
        results = await asyncio.gather(*tasks)
        total_duration = time.perf_counter() - start_time

    successes = [r for r in results if r[0] == 201]
    latencies = [r[1] for r in results if r[1] > 0]
    avg_latency_ms = (sum(latencies) / len(latencies)) * 1000 if latencies else 0
    throughput = len(successes) / total_duration

    report = {
        "metric_type": "backend_to_db",
        "load": iterations,
        "concurrency": concurrency,
        "throughput": round(throughput, 2),
        "latency_ms": round(avg_latency_ms, 2),
        "success_rate": round(len(successes) / iterations, 2),
    }

    print(f"REPORT_START{json.dumps(report)}REPORT_END")

    logger.info("Backend-to-DB stress test complete.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run Backend-to-DB benchmark")
    parser.add_argument(
        "--iterations",
        type=int,
        default=int(os.environ.get("CORE_QUEUE_LIMIT", "1000")),
        help="Number of iterations to run",
    )
    parser.add_argument(
        "--concurrency",
        type=int,
        default=50,
        help="Concurrency limit",
    )
    args = parser.parse_args()
    asyncio.run(run_benchmark(args.iterations, args.concurrency))
