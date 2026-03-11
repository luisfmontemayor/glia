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
          "Glia Error: API_INGEST_URL environment variable is not set and no base_url was provided. ",
          "Telemetry cannot be sent without a target endpoint."
        )
      }

      self$base_url <- target_url
      self$timeout <- as.numeric(timeout)
    },

    send_job_run = function(payload) {
      if (is.null(payload)) return(FALSE)

      json_str <- as.character(jsonlite::toJSON(payload, auto_unbox = TRUE))

      result <- tryCatch({
        # Rust FFI
        push_telemetry(json_str, self$base_url, self$timeout)
      }, error = function(e) {
        warning(paste("Rust FFI Error:", e$message))
        return(NULL)
      })

      if (is.null(result)) return(FALSE)

      if (result$status >= 200 && result$status < 300) {
        return(TRUE)
      } else {
        warning(paste("Glia Backend Error:", result$status, "-", result$body))
        return(FALSE)
      }
    }
  )
)