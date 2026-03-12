#' @importFrom R6 R6Class
#' @importFrom jsonlite toJSON
GliaClient <- R6::R6Class("GliaClient",
  public = list(
    base_url = NULL,
    timeout = NULL,

    initialize = function(
      base_url = NULL,
      timeout = 10.0
    ) {
      target_url <- base_url
      if (is.null(target_url) || target_url == "") {
        target_url <- Sys.getenv("API_INGEST_URL")
      }

      if (is.null(target_url) || target_url == "") {
        stop(
          "[GLIAR] API_INGEST_URL environment variable is not set and no base_url was provided. ",
          "Telemetry cannot be sent without a target endpoint."
        )
      }

      self$base_url <- target_url
      self$timeout <- as.numeric(timeout)
    },

    send_job_run = function(payload) {
      if (is.null(payload)) return(FALSE)

      json_str <- as.character(jsonlite::toJSON(payload, auto_unbox = TRUE))

      tryCatch({
        # Rust FFI (Non-blocking)
        res <- queue_telemetry(json_str, self$base_url, self$timeout)
        if (is.list(res) && isTRUE(res$success)) {
          return(TRUE)
        } else {
          warning(paste("[GLIAR] Failed to queue telemetry:", res$error))
          return(FALSE)
        }
      }, error = function(e) {
        warning(paste("[GLIAR] Could not queue telemetry:", e$message))
        return(FALSE)
      })
    },

    flush = function() {
      tryCatch({
        # Rust FFI (Blocking)
        flush_queue()
      }, error = function(e) {
        warning(paste("[GLIAR] Error during flush:", e$message))
        return(NULL)
      })
    }
  )
)