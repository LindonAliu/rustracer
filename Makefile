##
## EPITECH PROJECT, 2023
## Makefile
## File description:
## Rust raytracer epitech
##

all:
	cargo build --release
	cp target/release/raytracer .

clean:
	cargo clean

fclean: clean
	rm -f target/release/raytracer
	rm -f raytracer

re: fclean all

cargo:
	cargo run --release