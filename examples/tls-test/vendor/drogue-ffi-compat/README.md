# `drogue-ffi-compat`

A small utility library encompassing C-compatibility bits for making FFI integrations easier on non-libc platforms, such as Cortex-M.

## Variadics

Support for C-style variadic functions is provided through a `VaList` struct, which knows how to manipulate memory the same as C in order to peel off variable length argument lists.
It does *not* support direct implementation of variadic functions.
In order to use it with true variadic functions, a bonafide C shim is required in order to construct a `va_list`, using normal `va_start(...)` and `va_end(...)` macros.

## `*printf(...)`

This crate exports two `printf` compatible functions:

* `snprintf()` implemented in bonafide C
* `vsnprintf()` implemented in Rust.

Additionally, a minimal set of C-style printf formatting specifiers are marginally supported. 

* '%c'
* '%s'
* '%d'
* '%x' and '%X'

Width and precision modifiers are consumed, but not currently respected. 
The goal is to support these functions _well enough_ for simple integrations.
