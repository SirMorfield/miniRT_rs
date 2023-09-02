use std::os::raw::c_char;
use std::os::raw::c_int;

extern "C" {
    pub fn main2(argc: c_int, argv: *mut *mut c_char) -> c_int;
}
