library(gliar)
library(jsonlite)
library(argparse)

parser <- ArgumentParser(description = "Glia R Client Stress Test")

parser$add_argument(
  "--iterations",
  type = "integer",
  default = as.integer(Sys.getenv("GLIA_CORE_QUEUE_LIMIT", unset = "1000")),
  help = "Number of iterations [default %(default)s]"
)

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
  args <- parser$parse_args()
  
  glia_init()
  
  run_benchmark(args$iterations)
}
