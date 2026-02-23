library(testthat)
library(gliar)
library(httr2)
library(uuid)

test_that("R Client E2E: telemetry reaches backend", {
  ingest_url <- Sys.getenv("API_INGEST_URL")
  if (ingest_url == "") ingest_url <- "http://localhost:8000/ingest"
  
  base_url <- sub("/ingest$", "", ingest_url)
  
  unique_id <- substr(uuid::UUIDgenerate(), 1, 6)
  unique_name <- paste0("e2e_r_", unique_id)
  
  glia_init(api_url = ingest_url, app_name = unique_name)
  
  glia_track({
    res <- 1 + 1
    Sys.sleep(0.1) # Simulate minimal work
  }, tags = list(e2e = "true", client = "R"))
  
  Sys.sleep(0.5)
  
  resp <- request(base_url) |> 
    req_url_path("/telemetry") |> 
    req_perform()
  
  expect_equal(resp_status(resp), 200)
  
  jobs <- resp_body_json(resp)
  expect_true(length(jobs) > 0)
  
  found_job <- NULL
  for (job in rev(jobs)) {
    if (grepl(unique_id, job$program_name)) {
      found_job <- job
      break
    }
  }
  
  expect_false(is.null(found_job), label = paste("Job", unique_id, "not found in DB"))
  expect_match(found_job$program_name, unique_name)
  expect_equal(found_job$meta$e2e, "true")
  expect_equal(found_job$meta$client, "R")
  
  expect_gt(found_job$wall_time_sec, 0)
})
