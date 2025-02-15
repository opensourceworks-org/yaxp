.PHONY: run build  docker test docker-with-tests latest

#include .env
#export

VERSION := $(shell cat VERSION.txt)
DIST := dist
APP := yaxp-cli
TARGET_ARCH := arm64
TARGET_OS := linux

clean:
	@cargo clean

run:
	@cargo run -- --help

build:
	@cargo build --release

latest:
	git tag -f latest -m "chore: release latest"
	git push origin latest --force