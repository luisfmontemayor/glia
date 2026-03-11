library(testthat)
library(mockery)
library(gliar)

test_that("GliaClient sends valid payload successfully", {
  mock_ffi <- mock(list(status = 200, body = "OK"))
  client <- GliaClient$new(base_url = "http://test-api/injest")
  
  stub(client$send_job_run, "push_telemetry", mock_ffi)
  
  payload <- list(run_id = "123", cpu_percent = 50)
  success <- client$send_job_run(payload)
  
  expect_true(success)
})

test_that("GliaClient handles backend errors gracefully", {
  mock_ffi <- mock(list(status = 500, body = "Internal Server Error"))
  client <- GliaClient$new()
  stub(client$send_job_run, "push_telemetry", mock_ffi)
  
  expect_warning(
    client$send_job_run(list(data = 1)),
    "Glia Backend Error: 500"
  )
})

test_that("GliaClient handles FFI/Rust panics gracefully", {
  mock_ffi <- mock(stop("Rust Panic"))
  client <- GliaClient$new()
  stub(client$send_job_run, "push_telemetry", mock_ffi)
  
  expect_warning(
    client$send_job_run(list(data = 1)),
    "Rust FFI Error: Rust Panic"
  )
})