#!/usr/bin/env -S python
# Written by Luis Felipe Montemayor, sometime around March of 2026
import datetime
import json
import os
import re
import subprocess
import sys

from common.logs import setup_logger
from common.system import run_command

logger = setup_logger("GLIA_BENCHMARKER")

ITERATIONS_LIST = [100, 200, 500, 800, 1000]

BENCHMARKS = [
    {
        "name": "Client-to-Core (Python)",
        "cmd": "python benchmark_client2core.py",
    },
    {
        "name": "Client-to-Core (R)",
        "cmd": "Rscript benchmark_client2core.r",
    },
    {"name": "API Ingestion (Python Async HTTP)", "cmd": "python benchmark_core2db.py"},
    {"name": "API Ingestion (Rust Core Batching)", "cmd": "python benchmark_gcore2db_gcore.py"},
]


def check_infrastructure():
    """Checks if the API and Database are ready using mise tasks."""
    logger.info("Verifying Infrastructure (API & DB)...")

    # api:status returns 0 if healthy, which run_command returns as "0" when capture=False
    status = run_command(["mise", "run", "api:status"], capture=False)

    if status != "0":
        logger.error("Infrastructure check FAILED.")
        logger.error(
            "Ensure the API is running ('mise api:up') and the database is accessible."
        )
        return False

    logger.info("Infrastructure is healthy. Proceeding with benchmarks.")
    return True


def run_benchmark(cmd: str, iterations: int):
    """
    Executes a benchmark script with specific load parameters via environment.
    Captures stdstreams to extract the evaluation report.
    """
    env = os.environ.copy()
    env["CORE_QUEUE_LIMIT"] = str(iterations)
    env["CORE_LOCAL_QUEUE_LIMIT"] = str(iterations * 10)
    env["PERFORMANCE_TEST"] = "true"

    try:
        # Determine the directory of the current script to run commands from there
        script_dir = os.path.dirname(os.path.abspath(__file__))

        # We use shell=True to support pytest/Rscript directly as provided in BENCHMARKS
        result = subprocess.run(
            cmd,
            shell=True,
            capture_output=True,
            text=True,
            env=env,
            cwd=script_dir,
            check=False,  # Allow parsing even if assertions fail in performance tests
        )

        output = result.stdout
        # Extract the JSON report from the standard stream
        match = re.search(r"REPORT_START(\{.*?\})REPORT_END", output)
        if match:
            return json.loads(match.group(1))

        logger.error(f"Failed to find REPORT block in output for command: {cmd}")
        if result.stderr:
            logger.debug(f"Command error: {result.stderr.strip()}")
        return None

    except Exception as e:
        logger.error(f"Unexpected error executing {cmd}: {e}")
        return None


def main():
    if not check_infrastructure():
        sys.exit(1)

    performance_profile = {}

    for b in BENCHMARKS:
        benchmark_name = b["name"]
        performance_profile[benchmark_name] = {"latency": [], "throughput": []}

        for iterations in ITERATIONS_LIST:
            # Printing the current test being performed using the common logger
            logger.info(f"Performing Test: {benchmark_name} | Load: {iterations}")

            report = run_benchmark(b["cmd"], iterations)

            if report and "latency_ms" in report:
                performance_profile[benchmark_name]["latency"].append(
                    report["latency_ms"]
                )
                performance_profile[benchmark_name]["throughput"].append(
                    report.get("throughput", "ERR")
                )
            else:
                performance_profile[benchmark_name]["latency"].append("ERR")
                performance_profile[benchmark_name]["throughput"].append("ERR")

    # Generate the aligned Evaluation Report
    logger.info("=" * 80)
    logger.info("GLIA INFLECTION POINT CHARACTERIZATION")
    logger.info("=" * 80)

    # Define a fixed width for each column
    col_width = 12
    header_width = 18

    # Format the independent variables (ITERATIONS_LIST) once
    independent_str = "".join([f"{str(x):>{col_width}}" for x in ITERATIONS_LIST])

    print(f"{'Number of Jobs:':<{header_width}}{independent_str}")
    print("-" * 80)
    for name, metrics in performance_profile.items():
        print(f"## Benchmark:   {name}")

        # Format Latency
        formatted_latencies = []
        for val in metrics["latency"]:
            if isinstance(val, (int, float)):
                formatted_latencies.append(f"{val:>{col_width}.2f}")
            else:
                formatted_latencies.append(f"{str(val):>{col_width}}")

        # Format Throughput
        formatted_throughputs = []
        for val in metrics["throughput"]:
            if isinstance(val, (int, float)):
                formatted_throughputs.append(f"{val:>{col_width}.2f}")
            else:
                formatted_throughputs.append(f"{str(val):>{col_width}}")

        latency_str = "".join(formatted_latencies)
        throughput_str = "".join(formatted_throughputs)

        print(f"{'Latency (ms):':<{header_width}}{latency_str}")
        print(f"{'Throughput (j/s):':<{header_width}}{throughput_str}")
        print("-" * 80)

    # Benchmark Library Implementation
    library_path = os.path.join(os.path.dirname(os.path.abspath(__file__)), "benchmark_library.json")
    try:
        git_hash = subprocess.getoutput("git rev-parse HEAD").strip()
    except Exception:
        git_hash = "unknown"
        
    setup_type = os.environ.get("BENCHMARK_SETUP_TYPE", "local")
    
    # We use timezone-aware UTC datetime
    now_utc = datetime.datetime.now(datetime.timezone.utc)
    record = {
        "timestamp": now_utc.isoformat(),
        "git_commit": git_hash,
        "runtime": f"Python {sys.version.split()[0]}",
        "setup_type": setup_type,
        "performance_profile": performance_profile
    }
    
    library_data = []
    if os.path.exists(library_path):
        try:
            with open(library_path, "r") as f:
                library_data = json.load(f)
        except Exception as e:
            logger.error(f"Could not read existing benchmark library: {e}")
            
    library_data.append(record)
    
    try:
        with open(library_path, "w") as f:
            json.dump(library_data, f, indent=2)
        logger.info(f"Appended benchmark results to {library_path}")
    except Exception as e:
        logger.error(f"Failed to write benchmark library: {e}")


if __name__ == "__main__":
    main()
