use std::os::raw::c_char;
use std::os::raw::c_int;
mod helpers;
use helpers::*;

extern {
	fn mainj(
        argc: c_int,
        argv: *mut *mut c_char,
    ) -> c_int;

}

fn main() {
	let result: i32;
    unsafe {
		let argv = std::env::args().collect::<Vec<_>>();
		for arg in &argv {
			println!("The arg is: {}", arg);
		}
        result = mainj(argv.len() as i32, vector_to_cstring_vector(argv));
    }
    println!("The result from C is: {}", result);
}