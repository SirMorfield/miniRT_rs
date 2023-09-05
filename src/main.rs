mod helpers;
use helpers::*;
mod resolution;
use resolution::Resolution;
mod frame_buffer;
use frame_buffer::FrameBuffer;
mod foreign_function_interfaces;
mod vector;
use foreign_function_interfaces::*;
use std::time;
extern crate bmp;
mod scene;
use scene::Scene;
use std::path::Path;

use scene::Renderer;

fn cpp_main() -> i32 {
    let result: i32;
    unsafe {
        let argv = std::env::args().collect::<Vec<_>>();
        result = main2(argv.len() as i32, vector_to_cstring_vector(argv));
    }
    return result;
}

fn main() {
    let resolution = Resolution::new(100, 100, 4).unwrap();
    let mut frame_buffer = FrameBuffer::new(resolution).unwrap();
    let scene_path = Path::new("rt/dragon.rt");
    let scene = Scene::new(scene_path).unwrap();
    let renderer = Renderer::new(resolution);

    loop {
        match frame_buffer.get_coordinate() {
            None => break,
            Some((x, y)) => {
                let color = renderer.render(&scene, &scene.camera, x as f32, y as f32);
                frame_buffer.set_pixel(x, y, color);
                // print!("{} {} {}\r", x, y, color.x);
            }
        }
    }

    let path = Path::new("output.bmp");
    frame_buffer.save_as_bmp(path).unwrap();

    let start = time::Instant::now();
    let exit_code = cpp_main();
    let duration = start.elapsed();
    println!("Exit code: {}", exit_code);
    println!("Time elapsed: {}", duration.as_formatted_str());
}
