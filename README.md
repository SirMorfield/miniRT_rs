# My first Rust project: a ray tracer

It's based on https://github.com/SirMorfield/miniRT in CPP.

This image was rendered by this code:
![dragon](./media/dragon.png)

## Build and run
```bash
make build-run-release
```

```shell
# server
RUST_BACKTRACE=1 cargo run NetServer obj/teapot.obj output.bmp 127.0.0.1:6969

# client
RUST_BACKTRACE=1 cargo run NetClient obj/teapot.obj output.bmp 127.0.0.1:6969

while true; do RUST_BACKTRACE=1 cargo run NetClient obj/teapot.obj output.bmp 127.0.0.1:6969; sleep 2; done
```
