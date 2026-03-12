library(testthat)
library(gliar)

ITERATIONS = 1000
STRESS_LOAD = as.integer(Sys.getenv("GLIA_CORE_QUEUE_LIMIT", unset = "1000"))

get_stress_config <- function() {
  list(
    load_factor = STRESS_LOAD * 5,
    iterations = ITERATIONS,
    target_url = Sys.getenv("API_INGEST_URL", unset = "http://localhost:8000/ingest")
  )
}

test_that("client_to_core_overhead stress test", {
  skip_if_not(Sys.getenv("GLIA_PERFORMANCE_TEST") == "true", "Skipping performance test")
  
  config <- get_stress_config()
  iterations <- config$iterations
  stress_load <- config$load_factor
  
  message(sprintf("[GLIAR STRESS TEST] Starting stress test: %d jobs (Load Factor: %d)", iterations, stress_load))
  
  start_time <- proc.time()[["elapsed"]]
  
  for (i in 1:iterations) {
    glia_track("stress_worker", {
      NULL
    }, context = list(iteration = i, stress_load = stress_load))
  }
  
  end_time <- proc.time()[["elapsed"]]
  
  total_duration <- end_time - start_time
  avg_overhead_ms <- (total_duration / iterations) * 1000
  throughput <- iterations / total_duration
  
  message("[GLIAR STRESS TEST] Client-to-Core stress test complete.")
  message(sprintf("[GLIAR STRESS TEST] Duration: %.2fs", total_duration))
  message(sprintf("[GLIAR STRESS TEST] Avg. Glia overhead: %.4fms", avg_overhead_ms))
  message(sprintf("[GLIAR STRESS TEST] Measured Throughput: %.2f jobs/sec", throughput))
  
  expect_lt(avg_overhead_ms, 5.0)
})