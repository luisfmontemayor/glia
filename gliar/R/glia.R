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
#' @param api_url The URL of the Glia API backend. Defaults to the `API_URL`
#'   environment variable or "http://localhost:8000".
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
glia_init <- function(api_url = Sys.getenv("API_URL"),
                      app_name = NULL,
                      app_version = NULL,
                      tags = list()) {
  
  if (api_url == "") {
    stop(
      "[Glia Error] API endpoint not found. \n",
      "Please set the API_URL environment variable (e.g., in mise.toml) ",
      "or provide it directly to glia_init(api_url = '...').",
      call. = FALSE
    )
  }

  .glia_env$client <- GliaClient$new(base_url = api_url)
  .glia_env$app_name <- app_name
  .glia_env$app_version <- app_version
  .glia_env$tags <- tags
  
  message(paste("[Glia] Initialized tracking to:", api_url))
  invisible(NULL)
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
  context <- list(
    name = name %||% .glia_env$app_name,
    version = version %||% .glia_env$app_version,
    description = description,
    tags = all_tags
  )
  context <- context[!sapply(context, is.null)]

  tracker <- SystemTracker$new(context = context)
  tracker$start()

  exit_code <- 0
  on.exit({
    metrics <- tracker$capture(exit_code = exit_code)

    final_name <- name %||% .glia_env$app_name
    if (!is.null(final_name)) {
      metrics$program_name <- final_name
    }
    
    metrics$program_version <- version %||% .glia_env$app_version

    .glia_env$client$send_job_run(metrics)
  })

  tryCatch({
    rlang::eval_tidy(expr_quo)
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
#'   return(sum(1:n))
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
    stop("`f` must be a function.", call. = FALSE)
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