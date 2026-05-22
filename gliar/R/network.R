#' @importFrom R6 R6Class
#' @importFrom jsonlite toJSON
GliaClient <- R6::R6Class("GliaClient",
  public = list(
    base_url = NULL,
    timeout = NULL,

    initialize = function(
      base_url = NULL,
      timeout = NULL
    ) {
      target_url <- base_url
      if (is.null(target_url) || target_url == "") {
        target_url <- Sys.getenv("GLIA_API_URL")
      }

      if (is.null(target_url) || target_url == "") {
        # Don't stop here. Warn-and-drop will be handled at flush/queue time.
        target_url <- NULL
      }

      self$base_url <- target_url
      self$timeout <- if (is.null(timeout)) 10.0 else as.numeric(timeout)
    },

    send_job_run = function(payload) {
      if (is.null(payload)) return(FALSE)
      
      if (is.null(self$base_url) || self$base_url == "") {
        warning("[GLIAR] API endpoint not found. Telemetry will be dropped.", call. = FALSE)
        return(FALSE)
      }

      # The backend now only accepts batches (list of jobs)
      json_str <- paste0("[", as.character(jsonlite::toJSON(payload, auto_unbox = TRUE)), "]")

      tryCatch({
        # Rust FFI (Non-blocking)
        res <- enqueue_to_background(json_str, self$base_url, self$timeout)
        if (is.list(res) && isTRUE(res$success)) {
          TRUE
        } else {
          warning(paste("[GLIAR] Failed to queue telemetry:", res$error))
          FALSE
        }
      }, error = function(e) {
        warning(paste("[GLIAR] Could not queue telemetry:", e$message))
        FALSE
      })
    },

    flush = function() {
      if (is.null(self$base_url) || self$base_url == "") {
        return(NULL)
      }
      tryCatch({
        # Rust FFI (Blocking)
        flush_queue()
      }, error = function(e) {
        warning(paste("[GLIAR] Error during flush:", e$message))
        NULL
      })
    }
  )
)