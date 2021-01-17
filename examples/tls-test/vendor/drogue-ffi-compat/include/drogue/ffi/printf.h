
#include <stdarg.h>
#include <stddef.h>

extern int vsnprintf(char * restrict str, size_t size, const char * restrict format, va_list ap);
extern int snprintf(char * restrict str, size_t size, const char * restrict fmt, ...);