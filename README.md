# AeroPi

Autonomous quadcopter flight controller in Rust using a Raspberry Pi.

### Hardware

* Raspberry Pi 3

### Dependencies

* Rustup with the ARMv7 Linux target (`rustup target add armv7-unknown-linux-gnueabihf`)
* arm-linux-gnueabihf gcc cross-compiler (arm-linux-gnueabihf-gcc on the AUR for Arch Linux)
* Raspbian Stretch Lite

### Hardware Setup

* Status LED on GPIO 17
* HC SR04 ultrasonic distance sensor on GPIO 23 and 24

### Setup

Set up a new microSD card with Raspbian:

    ./scripts/raspbian.sh <SDCARD-DEV>

Configure using Ansible after booting:

    make config

Log in via ssh after booting:

    ssh pi@aeropi

Use the Makefile to deploy and run:

    make deploy run

Or run without deploying:

    make run

### Inspiration/Existing Works

* PiStuffing (http://blog.pistuffing.co.uk/category/pidrone and https://github.com/PiStuffing/Quadcopter)
