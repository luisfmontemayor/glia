#' @importFrom R6 R6Class
#' @importFrom ps ps_handle ps_cpu_times ps_memory_info
#' @importFrom uuid UUIDgenerate
#' @importFrom digest digest
#' @importFrom jsonlite toJSON
SystemTracker <- R6::R6Class("SystemTracker",
  public = list(
    process = NULL,
    start_time = NULL,
    cpu_start = NULL,
    run_id = NULL,
    user_meta = NULL,
    script_path = NULL,
    
    initialize = function(context = list()) {
      self$process <- ps::ps_handle()
      self$user_meta <- context
      self$run_id <- uuid::UUIDgenerate()
      
      # Attempt to detect script path (basic heuristic for Rscript)
      args <- commandArgs(trailingOnly = FALSE)
      file_arg <- grep("--file=", args, value = TRUE)
      if (length(file_arg) > 0) {
        self$script_path <- sub("--file=", "", file_arg)
      }
    },
    
    start = function() {
      self$start_time <- Sys.time()
      self$cpu_start <- ps::ps_cpu_times(self$process)
    },
    
    capture = function(exit_code = 0) {
      if (is.null(self$start_time)) stop("Tracker not started.")
      
      end_time <- Sys.time()
      cpu_end <- ps::ps_cpu_times(self$process)
      
      wall_time <- as.numeric(difftime(end_time, self$start_time, units = "secs"))
      
      cpu_total_start <- self$cpu_start[["user"]] + self$cpu_start[["system"]]
      cpu_total_end <- cpu_end[["user"]] + cpu_end[["system"]]
      cpu_consumed <- cpu_total_end - cpu_total_start
      
      cpu_percent <- 0.0
      if (wall_time > 0.0001) {
        cpu_percent <- (cpu_consumed / wall_time) * 100
      }
      
      rss_bytes <- ps::ps_memory_info(self$process)[["rss"]]
      max_rss_mb <- rss_bytes / (1024^2)
      
      sha <- "unknown-hash"
      if (!is.null(self$script_path) && file.exists(self$script_path)) {
        sha <- digest::digest(self$script_path, file = TRUE, algo = "sha256")
      }
      
      prog_name <- "r_client"
      if (!is.null(self$script_path)) {
        prog_name <- basename(self$script_path)
      }

      list(
        run_id = self$run_id,
        program_name = prog_name,
        user_name = Sys.info()[["user"]],
        script_sha256 = sha,
        hostname = Sys.info()[["nodename"]],
        os_info = paste(Sys.info()[["sysname"]], Sys.info()[["release"]]),
        script_path = self$script_path,
        argv = commandArgs(trailingOnly = TRUE),
        wall_time_sec = wall_time,
        started_at = format(self$start_time, "%Y-%m-%dT%H:%M:%SZ", tz = "UTC"),
        ended_at = format(end_time, "%Y-%m-%dT%H:%M:%SZ", tz = "UTC"),
        cpu_time_sec = cpu_consumed,
        cpu_percent = round(cpu_percent, 2),
        max_rss_mb = round(max_rss_mb, 2),
        exit_code_int = as.integer(exit_code),
        meta = self$user_meta
      )
    }
  )
)
