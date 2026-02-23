library(testthat)
library(mockery)
library(gliar)

test_that("SystemTracker captures CPU and Wall time correctly", {
  # 1. Setup Mocks
  # Mock ps::ps_cpu_times to return specific user/system times (Start: 0/0, End: 8/2)
  mock_cpu <- mock(
    list(user = 0.0, system = 0.0), # Start
    list(user = 8.0, system = 2.0)  # End
  )
  
  # Mock Sys.time to simulate 20 seconds passing
  t0 <- as.POSIXct("2025-01-01 12:00:00", tz = "UTC")
  t1 <- as.POSIXct("2025-01-01 12:00:20", tz = "UTC")
  mock_time <- mock(t0, t1)
  
  # Mock memory info (100 MB)
  # 100 * 1024^2 bytes
  mock_mem <- mock(list(rss = 104857600))
  
  # 2. Initialize Tracker
  tracker <- SystemTracker$new()
  
  # 3. Apply Mocks (stubbing the specific methods on this instance)
  stub(tracker$start, "ps::ps_cpu_times", mock_cpu)
  stub(tracker$start, "Sys.time", mock_time)
  
  stub(tracker$capture, "ps::ps_cpu_times", mock_cpu)
  stub(tracker$capture, "Sys.time", mock_time)
  stub(tracker$capture, "ps::ps_memory_info", mock_mem)
  
  # 4. Execution
  tracker$start()
  metrics <- tracker$capture()
  
  # 5. Assertions
  expect_equal(metrics$wall_time_sec, 20.0)
  expect_equal(metrics$cpu_time_sec, 10.0) # (8+2) - (0+0)
  expect_equal(metrics$cpu_percent, 50.0)  # 10s CPU / 20s Wall * 100
  expect_equal(metrics$max_rss_mb, 100.0)
})

test_that("SystemTracker detects script path and calculates SHA256", {
  # Mock file existence and digest
  mock_exists <- mock(TRUE)
  mock_digest <- mock("test-sha-hash")
  
  tracker <- SystemTracker$new()
  
  # Force set script path (since we can't easily mock commandArgs in R6 init post-hoc)
  tracker$script_path <- "/tmp/test_script.R"
  
  stub(tracker$capture, "file.exists", mock_exists)
  stub(tracker$capture, "digest::digest", mock_digest)
  # Stub deps to avoid errors
  stub(tracker$capture, "Sys.time", Sys.time)
  stub(tracker$capture, "ps::ps_cpu_times", list(user=1, system=1))
  stub(tracker$capture, "ps::ps_memory_info", list(rss=100))
  
  # Manually start to satisfy check
  tracker$start_time <- Sys.time()
  tracker$cpu_start <- list(user=0, system=0)
  
  metrics <- tracker$capture()
  
  expect_equal(metrics$script_path, "/tmp/test_script.R")
  expect_equal(metrics$script_sha256, "test-sha-hash")
})

test_that("capture throws error if not started", {
  tracker <- SystemTracker$new()
  expect_error(tracker$capture(), "Tracker not started")
})
