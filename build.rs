extern crate cc;
use glob::glob;

fn main() {
	let paths : Vec<String> = glob("./cpp_src/**/*.cpp")
		.unwrap()
		.map(|path| path.unwrap().display().to_string())
		.collect();

	for path in &paths {
		println!("cargo:rerun-if-changed={}", path);
	}

	cc::Build::new()
		.cpp(true)
		.std("c++17")
		.files(paths)
		.include("./cpp_include")
		.flag("-Wall")
		.flag("-Wextra")
		.flag("-Werror")
		.compile("libmain.a");
}