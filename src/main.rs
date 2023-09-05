mod helpers;
use helpers::*;
mod resolution;
use resolution::Resolution;
mod frame_buffer;
use frame_buffer::FrameBuffer;
mod vector;
use vector::Vec3;

mod foreign_function_interfaces;
use foreign_function_interfaces::*;
use std::time;
extern crate bmp;

fn cpp_main() -> i32 {
    let result: i32;
    unsafe {
        let argv = std::env::args().collect::<Vec<_>>();
        result = main2(argv.len() as i32, vector_to_cstring_vector(argv));
    }
    return result;
}

fn main() {
    let r = Resolution::new(1920, 1080, 4).unwrap();
    let mut f = FrameBuffer::new(r).unwrap();

    for x in 0..1000 {
        for y in 0..1000 {
            // ? are there better ways to cast values?
            f.set_pixel(x, y, Vec3::new(255, x as u8, y as u8));
        }
    }
    let path = std::path::Path::new("test.bmp");
    f.save_as_bmp(path).unwrap();

    let start = time::Instant::now();
    let exit_code = cpp_main();
    let duration = start.elapsed();
    println!("Exit code: {}", exit_code);
    println!("Time elapsed: {}", duration.as_formatted_str());
}
