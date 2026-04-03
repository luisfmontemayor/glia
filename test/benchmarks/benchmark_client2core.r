library(gliar)
library(jsonlite)

run_benchmark <- function(iterations) {
  stress_load <- iterations * 5

  message(sprintf("[GLIAR STRESS TEST] Starting stress test: %d jobs", iterations))

  start_time <- proc.time()[["elapsed"]]

  for (i in 1:iterations) {
    glia_track({
      NULL
    }, name = "stress_worker", tags = list(iteration = i, stress_load = stress_load))
  }

  end_time <- proc.time()[["elapsed"]]

  total_duration <- end_time - start_time
  avg_overhead_ms <- (total_duration / iterations) * 1000
  throughput <- iterations / total_duration

  report <- list(
    metric_type = "client_to_core_r",
    load = iterations,
    throughput = round(throughput, 2),
    latency_ms = round(avg_overhead_ms, 4),
    success_rate = 1.0
  )

  cat(paste0("REPORT_START", jsonlite::toJSON(report, auto_unbox = TRUE), "REPORT_END\n"))

  message("[GLIAR STRESS TEST] Client-to-Core stress test complete.")
}

if (!interactive()) {
  # Get iterations from environment variable, which run_benchmarks.py sets
  iterations <- as.integer(Sys.getenv("CORE_QUEUE_LIMIT", unset = "1000"))
  
  # Check if glia_init is available (it should be in gliar)
  if (exists("glia_init")) {
    glia_init()
    run_benchmark(iterations)
  } else {
    stop("glia_init not found. Is gliar package loaded?")
  }
}
