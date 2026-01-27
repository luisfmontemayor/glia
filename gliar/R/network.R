#' @importFrom R6 R6Class
#' @importFrom jsonlite toJSON
GliaClient <- R6::R6Class("GliaClient",
  public = list(
    base_url = NULL,
    timeout = NULL,

    initialize = function(
        base_url = Sys.getenv("GLIA_API_URL", "http://localhost:8000"),
        timeout = 10.0
    ) {
      self$base_url <- base_url
      self$timeout <- as.numeric(timeout)
    },

    send_job_run = function(payload) {
      if (is.null(payload)) return(FALSE)

      json_str <- as.character(jsonlite::toJSON(payload, auto_unbox = TRUE))

      result <- tryCatch({
        gliar::push_telemetry(json_str, self$base_url, self$timeout)
      }, error = function(e) {
        warning(paste("glia_core Error:", e$message))
        return(list(status = 0))
      })

      if (result$status >= 200 && result$status < 300) {
        return(TRUE)
      } else {
        warning(paste("Glia Backend Error:", result$status, "-", result$body))
        return(FALSE)
      }
    }
  )
)
