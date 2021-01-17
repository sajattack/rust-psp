#![cfg_attr(not(test), no_std)]

#![allow(non_camel_case_types)]
#![allow(dead_code)]

mod printf;
mod atoi;
mod variadic;
mod strlen;

pub use printf::vsnprintf as vsnprintf;
pub use printf::snprintf as snprintf;
pub use variadic::va_list as va_list;

