#' @keywords internal
"_PACKAGE"

#' @useDynLib gliar, .registration = TRUE
NULL

#' @export
push_telemetry <- function(json_payload, url, timeout) {
  .Call(wrap__push_telemetry, json_payload, url, timeout)
}