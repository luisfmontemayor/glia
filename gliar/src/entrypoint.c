#include <R.h>
#include <Rinternals.h>

extern void R_init_gliar_rs(DllInfo *dll);

void R_init_gliar(DllInfo *dll) {
    R_init_gliar_rs(dll);
}
