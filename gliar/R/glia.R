#' Glia Main Entry Point
#'
#' @description
#' The main class for interacting with the Glia observability suite.
#' Use `glia$track({})` to monitor code blocks.
#'
#' @export
Glia <- R6::R6Class("Glia",
  public = list(
    tracker = NULL,
    client = NULL,

    #' @description
    #' Initialize the Glia instance.
    initialize = function() {
      self$client <- GliaClient$new()
    },

    #' @description
    #' Track a block of code and push metrics to the backend.
    #'
    #' @param code The code block to execute and monitor.
    #' @param meta A named list of custom metadata to attach to the run.
    #' @examples
    #' glia$track({
    #'   Sys.sleep(1)
    #'   print("Job done")
    #' }, meta = list(env = "prod"))
    track = function(code, meta = list()) {
      self$tracker <- SystemTracker$new(context = meta)
      self$tracker$start()

      # This runs regardless of success or failure (like Python's 'finally')
      exit_code <- 0
      on.exit({
        metrics <- self$tracker$capture(exit_code)
        
        self$client$send_job_run(metrics)
      }, add = TRUE)

      tryCatch({
        force(code)
      }, error = function(e) {
        exit_code <<- 1
        stop(e) # Re-throw so the script actually fails
      })
    }
  )
)
