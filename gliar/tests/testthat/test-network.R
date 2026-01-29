library(testthat)
library(mockery)
library(gliar)

test_that("GliaClient sends valid payload successfully", {
  # Mock the Rust FFI function `push_telemetry`
  # It expects (json_str, url, timeout) and returns list(status, body)
  mock_ffi <- mock(list(status = 200, body = "OK"))
  
  client <- GliaClient$new(base_url = "http://test-api")
  
  # Stub the internal call inside send_job_run
  stub(client$send_job_run, "gliar::push_telemetry", mock_ffi)
  
  payload <- list(run_id = "123", cpu_percent = 50)
  success <- client$send_job_run(payload)
  
  expect_true(success)
  
  # Verify arguments passed to Rust
  args <- mock_args(mock_ffi)[[1]]
  expect_match(args[[1]], '"run_id":"123"') # JSON string
  expect_equal(args[[2]], "http://test-api") # URL
})

test_that("GliaClient handles backend errors gracefully", {
  # Simulate 500 Error
  mock_ffi <- mock(list(status = 500, body = "Internal Server Error"))
  
  client <- GliaClient$new()
  stub(client$send_job_run, "gliar::push_telemetry", mock_ffi)
  
  # Should expect a warning, not a crash
  expect_warning(
    success <- client$send_job_run(list(data = 1)),
    "Glia Backend Error: 500"
  )
  expect_false(success)
})

test_that("GliaClient handles FFI/Rust panics gracefully", {
  # Simulate an R error (which is how FFI panics might bubble up if not handled in C)
  mock_ffi <- mock(stop("Rust Panic"))
  
  client <- GliaClient$new()
  stub(client$send_job_run, "gliar::push_telemetry", mock_ffi)
  
  expect_warning(
    success <- client$send_job_run(list(data = 1)),
    "Rust FFI Error"
  )
  # Even if Rust crashes, R client returns FALSE (safe)
  expect_equal(success, list(status = 0)) 
})
