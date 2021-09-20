# enhash

CLI Tool for Encoding, Hashing.

## Installation

git clone https://gitlab.com/raakhul/enhash

cd enhash

cargo build --release

## Usage

- If input string has space between two words, enclose them within single quotes ('')

**Encoding:**
cargo run --release -- -e base64 'Hello World'

**Hashing:**
cargo run --release -- -s sha256 'Hello World'



