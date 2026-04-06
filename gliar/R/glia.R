#' @importFrom R6 R6Class
#' @import rlang

# Package-level environment to store the global client and settings
.glia_env <- new.env(parent = emptyenv())

#' Initialize the Glia client
#'
#' @description
#' Configures the package-level client that will be used for all tracking.
#' This should be called once when your application or script starts.
#'
#' @param api_url The URL of the Glia API backend. Defaults to the `GLIA_API_URL`
#'   environment variable or "http://host:8000/ingest".
#' @param app_name A global name for the application or project being tracked.
#' @param app_version A global version for the application.
#' @param tags A named list of global tags to be included with every tracked event.
#'
#' @export
#' @examples
#' \dontrun{
#' glia_init(
#'   api_url = "http://my-glia-instance:8000",
#'   app_name = "DataPipeline",
#'   app_version = "v2.1.0",
#'   tags = list(team = "data-science", priority = "high")
#' )
#' }
# TODO: adapt to base URL env var
glia_init <- function(api_url = NULL,
                      app_name = NULL,
                      app_version = NULL,
                      tags = list()) {
  
  target_url <- api_url
  if (is.null(target_url) || target_url == "") {
    target_url <- Sys.getenv("GLIA_API_URL")
  }

  if (is.null(target_url) || target_url == "") {
    stop(
      "[GLIAR] API endpoint not found. \n",
      "Please set the GLIA_API_URL environment variable (e.g., in mise.toml) ",
      "or provide it directly to glia_init(api_url = '...').",
      call. = FALSE
    )
  }

  .glia_env$client <- GliaClient$new(base_url = target_url)
  .glia_env$app_name <- app_name
  .glia_env$app_version <- app_version
  .glia_env$tags <- tags

  # Register finalizer to flush on exit
  reg.finalizer(.glia_env, function(e) {
    if (exists("client", envir = e) && !is.null(e$client)) {
      client <- e$client
      if (is.function(client$flush)) {
        client$flush()
      }
    }
  }, onexit = TRUE)
  
  message(paste("[GLIAR] Initialized tracking to:", api_url))
  invisible(NULL)
}

#' Flush all pending telemetry
#' @export
glia_flush <- function() {
  if (!is.null(.glia_env$client)) {
    .glia_env$client$flush()
  }
}
#' Track an R expression
#'
#' @description
#' Executes an R expression and records system metrics (wall time, CPU usage,
#' memory), sending them to the Glia backend. It uses `on.exit` to ensure
#' that metrics are sent even if the expression fails.
#'
#' @param expr The R expression to execute and track.
#' @param name A specific name for this tracked block. Overrides the global `app_name`
#'   and the script's filename.
#' @param version A specific version for this tracked block. Overrides `app_version`.
#' @param tags A named list of tags to add to this specific tracked event. These are
#'   merged with any global tags set by `glia_init()`.
#' @param description A string providing more context for what this block does.
#'
#' @return The result of the evaluated expression.
#' @export
#' @examples
#' \dontrun{
#' glia_init()
#'
#' # Track a simple block of code
#' glia_track({
#'   print("Doing some work...")
#'   Sys.sleep(1)
#'   a <- 1 + 1
#'   print("Work done.")
#' }, name = "data-processing", tags = list(step = "ingestion"))
#'
#' # It captures the return value
#' result <- glia_track({
#'   x <- 5 * 10
#'   x
#' })
#' print(result) # 50
#' }
glia_track <- function(expr, name = NULL, version = NULL, tags = list(), description = NULL) {
  if (is.null(.glia_env$client)) {
    glia_init()
  }

  expr_quo <- rlang::enquo(expr)
  all_tags <- c(.glia_env$tags, tags)
  
  # Flatten context for the 'meta' field
  context <- list(
    version = version %||% .glia_env$app_version,
    description = description
  )
  context <- c(context[!sapply(context, is.null)], all_tags)

  tracker <- SystemTracker$new(context = context)
  tracker$start()

  exit_code <- 0
  
  # Helper to send metrics
  send_metrics <- function(code) {
    metrics <- tracker$capture(exit_code = code)

    if (is.null(metrics$script_path) || length(metrics$script_path) == 0) {
      metrics$script_path <- "R_session"
    } else {
      metrics$script_path <- as.character(metrics$script_path)
    }

    final_name <- name %||% .glia_env$app_name %||% metrics$script_path
    metrics$program_name <- as.character(final_name)

    .glia_env$client$send_job_run(metrics)
  }

  # Ensure metrics are sent even on unexpected errors/interrupts
  # but try to do it explicitly for the success path to avoid race with glia_flush()
  state <- new.env(parent = emptyenv())
  state$sent <- FALSE
  
  on.exit({
    if (!state$sent) {
      send_metrics(exit_code)
    }
  })

  tryCatch({
    res <- rlang::eval_tidy(expr_quo)
    state$sent <- TRUE
    send_metrics(0)
    res
  }, error = function(e) {
    exit_code <<- 1
    stop(e)
  })
}

#' Wrap a function for tracking
#'
#' @description
#' Creates a new version of a function that, when called, will be automatically
#' tracked by `glia_track()`.
#'
#' @param f The function to wrap.
#' @param name A specific name for this tracked function. Defaults to the name
#'   of the function variable.
#' @param version A specific version for this tracked block. Overrides `app_version`.
#' @param tags A named list of tags to add to this specific tracked event.
#' @param description A string providing more context for what this function does.
#'
#' @return A new function that automatically tracks its execution.
#' @export
#' @examples
#' \dontrun{
#' glia_init()
#'
#' my_heavy_computation <- function(n) {
#'   Sys.sleep(0.5)
#'   sum(1:n)
#' }
#'
#' # Create a tracked version of the function
#' tracked_computation <- glia_wrap(my_heavy_computation, tags = list(type = "summation"))
#'
#' # Calling the new function executes it and sends the metrics
#' result <- tracked_computation(1000)
#' }
glia_wrap <- function(f, name = NULL, version = NULL, tags = list(), description = NULL) {
  if (!is.function(f)) {
    stop("[GLIAR] `f` must be a function.", call. = FALSE)
  }

  if (is.null(name)) {
    name <- deparse(substitute(f))
  }

  function(...) {
    expr_to_track <- rlang::quo(f(...))

    glia_track(!!expr_to_track,
      name = name,
      version = version,
      tags = tags,
      description = description
    )
  }
}