all:
	gcc -shared -o libhello.so src/main.c
	rustc src/main.rs
