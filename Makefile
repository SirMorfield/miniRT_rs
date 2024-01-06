build-run-release:
	cargo build --release --bin minirt_rs
	./target/release/minirt_rs rt/dragon.rt

build-run-debug:
	cargo +nightly build --bin minirt_rs
	./target/debug/minirt_rs obj/tree.obj

update-dragon:
	convert output.bmp media/dragon.png
