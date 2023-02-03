# hotplate
A DIY soldering hotplate based on an ESP32 written in Rust

## Setup
To use Rust on the EPS32 a custom fork of the Rust compiler is required!
Installation instructions for Arch Linux
```
pacman -S espup
espup install
. $HOME/export-esp.sh
```

We also need to install some tools for flashing the firmware
```
cargo install cargo-generate
cargo install ldproxy
cargo install espup
cargo install espflash
cargo install cargo-espflash
```
