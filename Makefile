build-run-release:
	cargo build --release --bin minirt_rs
	./target/release/minirt_rs rt/dragon.rt

build-run-debug:
	cargo build --bin minirt_rs
	./target/debug/minirt_rs rt/dragon.rt
