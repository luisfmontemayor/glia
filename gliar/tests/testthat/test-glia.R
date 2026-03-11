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
  glia_init(api_url = "http://test-api/injest")
  mock_client <- mock(TRUE)
  assign("client", list(send_job_run = mock_client), envir = gliar:::.glia_env)
  assign("SystemTracker", create_mock_tracker(), envir = ns)

  f <- function(x) x * 2
  tracked_f <- glia_wrap(f, name = "my_func")
  
  expect_equal(tracked_f(5), 10)
  expect_called(mock_client, 1)
})