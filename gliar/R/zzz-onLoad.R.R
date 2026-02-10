#' Glia Global Instance
#' @export
glia <- NULL

.onLoad <- function(libname, pkgname) {
  # Assign the instance to the package namespace
  # We use unlockBinding because 'glia' is exported and locked by default
  env <- asNamespace(pkgname)
  unlockBinding("glia", env)
  assign("glia", Glia$new(), envir = env)
  lockBinding("glia", env)
}
