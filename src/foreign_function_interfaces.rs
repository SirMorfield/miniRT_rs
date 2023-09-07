use std::os::raw::c_char;
use std::os::raw::c_int;

use crate::helpers::vector_to_cstring_vector;

#[allow(dead_code)]
extern "C" {
    pub fn main2(argc: c_int, argv: *mut *mut c_char) -> c_int;
}

#[allow(dead_code)]
fn cpp_main() -> i32 {
    let result: i32;
    unsafe {
        let argv = std::env::args().collect::<Vec<_>>();
        result = main2(argv.len() as i32, vector_to_cstring_vector(argv));
    }
    return result;
}
