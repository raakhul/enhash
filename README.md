# enhash

CLI Tool for Encoding, Hashing.

## Installation

git clone https://gitlab.com/raakhul/enhash

cd enhash

cargo build --release

## Usage

**Encoding:**

cargo run --release -- -e base64 'Hello World'

**Hashing:**

cargo run --release -- -s sha256 'Hello World'

- If input string has space between two words, enclose them within single quotes ('')

