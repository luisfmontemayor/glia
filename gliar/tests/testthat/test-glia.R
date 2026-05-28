library(testthat)
library(mockery)
library(gliar)
library(R6)

create_mock_tracker <- function(run_id = "test-run", exit_code = 0) {
  R6::R6Class("MockSystemTracker",
    public = list(
      initialize = function(context = NULL) {},
      start = function() {},
      capture = function(exit_code = 0) {
        list(run_id = run_id, exit_code_int = as.integer(exit_code))
      }
    )
  )
}
ns <- asNamespace("gliar")
if (bindingIsLocked("SystemTracker", ns)) unlockBinding("SystemTracker", ns)

test_that("glia_track executes code and sends metrics on success", {
  old_client <- gliar:::.glia_env$client
  old_tracker <- ns$SystemTracker
  withr::defer({
    assign("client", old_client, envir = gliar:::.glia_env)
    assign("SystemTracker", old_tracker, envir = ns)
  })
  glia_init(api_url = "http://test-api/injest")
  mock_client <- mock(TRUE)

  assign("client", list(send_job_run = mock_client), envir = gliar:::.glia_env)

  MockTracker <- create_mock_tracker(run_id = "track-success")
  assign("SystemTracker", MockTracker, envir = ns)

  result <- glia_track({ 1 + 1 })
  
  expect_equal(result, 2)
  expect_called(mock_client, 1)
})

test_that("glia_wrap handles function tracking correctly", {
  old_client <- gliar:::.glia_env$client
  old_tracker <- ns$SystemTracker
  withr::defer({
    assign("client", old_client, envir = gliar:::.glia_env)
    assign("SystemTracker", old_tracker, envir = ns)
  })
  glia_init(api_url = "http://test-api/injest")
  mock_client <- mock(TRUE)
  assign("client", list(send_job_run = mock_client), envir = gliar:::.glia_env)
  assign("SystemTracker", create_mock_tracker(), envir = ns)

  f <- function(x) x * 2
  tracked_f <- glia_wrap(f, name = "my_func")
  
  expect_equal(tracked_f(5), 10)
  expect_called(mock_client, 1)
})
test_that("glia_track does not crash when run without glia_init and missing env vars", {
  # Temporarily clear environment variable and global client
  old_url <- Sys.getenv("GLIA_API_URL")
  Sys.unsetenv("GLIA_API_URL")
  
  old_client <- gliar:::.glia_env$client
  assign("client", NULL, envir = gliar:::.glia_env)
  
  old_tracker <- ns$SystemTracker
  on.exit({
    if (old_url != "") Sys.setenv(GLIA_API_URL = old_url)
    assign("client", old_client, envir = gliar:::.glia_env)
    assign("SystemTracker", old_tracker, envir = ns)
  })

  # Need mock tracker to bypass rust FFI
  MockTracker <- create_mock_tracker(run_id = "track-missing")
  assign("SystemTracker", MockTracker, envir = ns)

  expect_warning({
    result <- glia_track({ 1 + 1 })
    expect_equal(result, 2)
  }, "API endpoint not found")
})
