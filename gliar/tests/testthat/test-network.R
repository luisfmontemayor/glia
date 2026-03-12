library(testthat)
library(mockery)
library(gliar)

test_that("GliaClient queues valid payload successfully", {
  # Mock returns a list indicating success in the new FFI
  mock_ffi <- mock(list(success = TRUE))
  client <- GliaClient$new(base_url = "http://test-api/injest")
  
  stub(client$send_job_run, "queue_telemetry", mock_ffi)
  
  payload <- list(run_id = "123", cpu_percent = 50)
  success <- client$send_job_run(payload)
  
  expect_true(success)
  expect_called(mock_ffi, 1)
})

test_that("GliaClient handles queueing errors gracefully", {
  mock_ffi <- mock(list(success = FALSE, error = "Queue Full"))
  client <- GliaClient$new()
  stub(client$send_job_run, "queue_telemetry", mock_ffi)
  
  expect_warning(
    success <- client$send_job_run(list(data = 1)),
    "\\[Glia\\] Failed to queue telemetry: Queue Full"
  )
  expect_false(success)
})

test_that("GliaClient handles FFI/Rust errors gracefully", {
  mock_ffi <- mock(stop("FFI Error"))
  client <- GliaClient$new()
  stub(client$send_job_run, "queue_telemetry", mock_ffi)
  
  expect_warning(
    client$send_job_run(list(data = 1)),
    "\\[Glia\\] Could not queue telemetry: FFI Error"
  )
})