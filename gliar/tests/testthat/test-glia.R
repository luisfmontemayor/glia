library(testthat)
library(mockery)
library(gliar)

test_that("glia$track executes code and sends metrics on success", {
  # Setup Global Glia
  local_glia <- Glia$new()
  
  # Mock internal components
  mock_tracker_capture <- mock(list(run_id = "test", exit_code_int = 0))
  mock_client_send <- mock(TRUE)
  
  # We need to stub SystemTracker$new to return a mock or stub the instance logic
  # Since R6 mocking of 'new' is hard, we stub the methods on the instance *inside* track?
  # Easier: Stub the 'tracker' and 'client' fields on our local_glia instance
  
  # 1. Mock the Client
  local_glia$client <- mock(send_job_run = mock_client_send)
  class(local_glia$client) <- "GliaClient" # Fake class for R6 if needed
  
  # 2. Mock the Tracker (Trickier because track() calls SystemTracker$new())
  # We will mock the SystemTracker generator itself using `stub` on the `track` method
  mock_tracker_instance <- list(
    start = mock(TRUE),
    capture = mock_tracker_capture
  )
  mock_tracker_cls <- mock(new = function(...) mock_tracker_instance)
  
  stub(local_glia$track, "SystemTracker", mock_tracker_cls)
  
  # 3. Run Track
  result <- local_glia$track({
    1 + 1
  })
  
  expect_equal(result, 2)
  
  # Verify capture called with exit_code 0
  expect_called(mock_tracker_capture, 1)
  expect_args(mock_tracker_capture, 1, exit_code = 0)
  
  # Verify client sent data
  expect_called(local_glia$client$send_job_run, 1)
})

test_that("glia$track captures error exit code when script fails", {
  local_glia <- Glia$new()
  
  # Mock Client
  local_glia$client$send_job_run <- mock(TRUE)
  
  # Mock Tracker using the same injection trick
  mock_capture <- mock(list(exit_code_int = 1))
  mock_tracker_instance <- list(
    start = mock(TRUE),
    capture = mock_capture
  )
  mock_tracker_cls <- mock(new = function(...) mock_tracker_instance)
  
  stub(local_glia$track, "SystemTracker", mock_tracker_cls)
  
  # Expect error to bubble up
  expect_error(
    local_glia$track({
      stop("User Script Failure")
    }),
    "User Script Failure"
  )
  
  # Verify capture was called with exit_code 1
  expect_called(mock_capture, 1)
  args <- mock_args(mock_capture)[[1]]
  expect_equal(args[[1]], 1) # exit_code argument
})
