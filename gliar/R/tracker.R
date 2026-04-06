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
    script_path = "R_session", # Default to string instead of NULL
    
    initialize = function(context = list()) {
      self$process <- ps::ps_handle()
      self$user_meta <- context
      self$run_id <- uuid::UUIDgenerate()
      
      # Attempt to detect script path (basic heuristic for Rscript)
      args <- commandArgs(trailingOnly = FALSE)
      file_arg <- grep("--file=", args, value = TRUE)
      if (length(file_arg) > 0) {
        # Ensure we only take one and it is a character string
        path <- sub("--file=", "", file_arg[1])
        if (nchar(path) > 0) {
          self$script_path <- path
        }
      }
    },
    
    start = function() {
      self$start_time <- Sys.time()
      self$cpu_start <- ps::ps_cpu_times(self$process)
    },
    
    capture = function(exit_code = 0) {
      if (is.null(self$start_time)) stop("[GLIAR] Tracker not started.")
      
      end_time <- Sys.time()
      cpu_end <- ps::ps_cpu_times(self$process)
      
      wall_time_sec <- as.numeric(difftime(end_time, self$start_time, units = "secs"))
      wall_time_ms <- as.integer(wall_time_sec * 1000)
      
      cpu_total_start <- self$cpu_start[["user"]] + self$cpu_start[["system"]]
      cpu_total_end <- cpu_end[["user"]] + cpu_end[["system"]]
      cpu_consumed <- cpu_total_end - cpu_total_start
      
      cpu_percent <- 0.0
      if (wall_time_sec > 0.0001) {
        cpu_percent <- (cpu_consumed / wall_time_sec) * 100
      }
      
      rss_bytes <- ps::ps_memory_info(self$process)[["rss"]]
      max_rss_kb <- as.integer(rss_bytes / 1024)
      
      # SHA fallback to empty string or known placeholder
      sha <- "" 
      if (!is.null(self$script_path) && 
          self$script_path != "R_session" && 
          file.exists(self$script_path)) {
        sha <- digest::digest(self$script_path, file = TRUE, algo = "sha256")
      }
      
      prog_name <- "r_client"
      if (!is.null(self$script_path) && self$script_path != "R_session") {
        prog_name <- basename(self$script_path)
      }

      # Return the list, ensuring all character fields are actual strings
      list(
        run_id = as.character(self$run_id),
        program_name = as.character(prog_name),
        user_name = as.character(Sys.info()[["user"]]),
        script_sha256 = as.character(sha),
        hostname = as.character(Sys.info()[["nodename"]]),
        os_info = as.character(paste(Sys.info()[["sysname"]], Sys.info()[["release"]])),
        script_path = as.character(self$script_path),
        argv = as.character(commandArgs(trailingOnly = TRUE)),
        wall_time_ms = as.integer(wall_time_ms),
        started_at = format(self$start_time, "%Y-%m-%dT%H:%M:%SZ", tz = "UTC"),
        ended_at = format(end_time, "%Y-%m-%dT%H:%M:%SZ", tz = "UTC"),
        cpu_time_sec = as.numeric(cpu_consumed),
        cpu_percent = round(as.numeric(cpu_percent), 2),
        max_rss_kb = as.integer(max_rss_kb),
        exit_code_int = as.integer(exit_code),
        meta = self$user_meta
      )
    }
  )
)