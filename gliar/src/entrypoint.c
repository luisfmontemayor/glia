#include <R.h>
#include <Rinternals.h>

extern void R_init_gliar_extendr(DllInfo *dll);

void R_init_gliar(DllInfo *dll) {
    R_init_gliar_extendr(dll);
}