#include <R.h>
#include <Rinternals.h>

extern void R_init_glia_core(DllInfo *dll);

void R_init_gliar(DllInfo *dll) {
    R_init_glia_core(dll);
}