library(testthat)
library(gliar)
library(httr2)
library(uuid)

test_that("R Client E2E: telemetry reaches backend", {
  raw_url <- Sys.getenv("GLIA_API_URL")
  if (raw_url == "" || is.na(raw_url)) {
    raw_url <- "http://localhost:8000"
  }
  
  base_url <- sub("/ingest/?$", "", raw_url)
  base_url <- sub("/$", "", base_url)
  ingest_url <- paste0(base_url, "/ingest")
  
  unique_id <- substr(uuid::UUIDgenerate(), 1, 6)
  unique_name <- paste0("e2e_r_", unique_id)
  
  glia_init(api_url = ingest_url, app_name = unique_name)
  
  glia_track({
    res <- 1 + 1
    Sys.sleep(0.1)
  }, tags = list(e2e = "true", client = "R"))
  
  # Wait for background worker to finish
  glia_flush()
  
  resp <- request(base_url) |> 
    req_url_path("/telemetry") |> 
    req_retry(max_tries = 3) |> 
    req_perform()
  
  expect_equal(resp_status(resp), 200)
  
  jobs <- resp_body_json(resp)
  expect_true(length(jobs) > 0)
  
  found_job <- NULL
  for (job in rev(jobs)) {
    if (grepl(unique_id, job[["program_name"]])) {
      found_job <- job
      break
    }
  }
  
  expect_false(
    is.null(found_job), 
    label = paste("Job with ID", unique_id, "not found in DB telemetry list")
  )
  
  expect_match(as.character(found_job[["program_name"]]), unique_name)
  
  meta <- found_job[["meta"]]
  
  # The model has meta as a flat dict[str, Any]
  expect_equal(as.character(meta[["e2e"]]), "true")
  expect_equal(as.character(meta[["client"]]), "R")
  
  expect_gt(as.numeric(found_job[["wall_time_ms"]]), 0)
  
  print(paste("[SUCCESS] E2E: Verified job", unique_name))
})