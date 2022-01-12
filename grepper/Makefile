.SILENT:
.PHONY: format test watch run clean


## help: this help file
help:
	@( echo "" && echo "Makefile targets..." && echo "" )
	@( cat Makefile | grep '^##' | sed -e 's/##/ -/' | sort && echo "" )

## format: run the mix formatter
format:
	cargo fmt --all

## release: build the distribution for production.
release:
	cargo build --release

## clean: remove all files from _build and deps
clean:
	cargo clean

## test: test the application
test:
	cargo test

## docs: build the docs
docs:
	cargo docs

## run: run the application against a known user/project
run:
	cargo run How poem.txt

## watch: watch test, lib, and config files and compile and test on change
watch:
	clear
	while true; do inotifywait -r -e modify src ; clear; make test; done

