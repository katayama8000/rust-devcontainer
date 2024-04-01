.PHONY: run test up

up:
	cd .devcontainer && docker-compose up -d && docker-compose exec rust_devcontainer bash

run:
	cargo run

test:
	cargo test
