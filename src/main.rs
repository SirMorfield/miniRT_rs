mod helpers;
use helpers::AsFormattedString;
use helpers::*;

mod foreign_function_interfaces;
use foreign_function_interfaces::*;
use std::time;

fn cpp_main() -> i32 {
    let result: i32;
    unsafe {
        let argv = std::env::args().collect::<Vec<_>>();
        result = main2(argv.len() as i32, vector_to_cstring_vector(argv));
    }
    return result;
}

fn main() {
    let start = time::Instant::now();
    let exit_code = cpp_main();
    let duration = start.elapsed();
    println!("Exit code: {}", exit_code);
    println!("Time elapsed: {}", duration.as_formatted_str());
}
