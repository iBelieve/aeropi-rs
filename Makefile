build:
	cargo build --target=arm-unknown-linux-gnueabihf

deploy: build
	rsync target/arm-unknown-linux-gnueabihf/debug/aeropi pi@aeropi:/aeropi/

run: deploy
	ssh pi@aeropi /aeropi/aeropi
