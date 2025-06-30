hello:
	# Hello Stranger!
	# choose one of the following commands
	#
	# to build owl you will need the following dependencies:
	# - rust-toolchain
	# 
	# `make debug`      to build the debug version of owl
	# `make release`    to build the release version of owl
	# `make install`    to install the release version of owl under the PREFIX
	#                   environment variable
	# `make deps`       to check if all needed dependecies are installed

PREFIX ?= /usr/local/bin

deps:
	cargo --version >/dev/null && echo "success"

debug: src/* Cargo.toml
	cargo build

release: src/* Cargo.toml
	cargo build --release

install:
	cp ./target/release/owl ${PREFIX}/owl

uninstall:
	rm -f ${PREFIX}/owl

clean:
	cargo clean
