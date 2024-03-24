flames:
	sudo cargo flamegraph --freq 1000 -- obj/teapot.obj output.bmp
	sudo chmod o+rw output.bmp
tea:
	rm -rf output.bmp
	cargo run --release -- ToFile obj/teapot.obj output.bmp

window:
	cargo run --release -- Window obj/teapot.obj

.PHONY: flames tea window
