mod helpers;
use helpers::*;

mod foreign_function_interfaces;
use foreign_function_interfaces::*;

fn main() {
    let result: i32;
    unsafe {
        let argv = std::env::args().collect::<Vec<_>>();
        for arg in &argv {
            println!("The arg is: {}", arg);
        }
        result = main2(argv.len() as i32, vector_to_cstring_vector(argv));
    }
    println!("The result from C is: {}", result);
}
