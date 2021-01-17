
#include "drogue/ffi/printf.h"

extern int snprintf(char * restrict str, size_t size, const char * restrict fmt, ...) {
    va_list ap;
    int n;

    va_start(ap,fmt);
    n=vsnprintf(str,size,fmt,ap);
    va_end(ap);

    return n;
}