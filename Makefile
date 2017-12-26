all: deploy run

build:
	cargo build

test:
	cargo test

deploy: build
	scp target/armv7-unknown-linux-gnueabihf/debug/aeropi pi@aeropi:~/

run:
	ssh pi@aeropi /home/pi/aeropi
