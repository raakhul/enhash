# enhash

CLI Tool for Encoding, Hashing.

## Installation

git clone https://github.com/raakhul/enhash

cd enhash

cargo build --release

## Usage

If input string has space between two words, enclose them within single quotes (' ').

**Ex:** Hello Rust -> 'Hello Rust'

**Encoding:**

cargo run --release -- -e base64 'Hello Rust'

**Hashing:**

cargo run --release -- -s sha256 'Hello Rust'
