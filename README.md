# converter tool for decimal, hexadecimal, and binary values

## Description
This repository contains a small command-line tool written in Rust that is designed to convert decimal, hexadecimal, and binary values.

## Features
- Convert decimal values to hexadecimal and binary.
- Convert hexadecimal values to decimal and binary.
- Convert binary values to decimal and hexadecimal.

## Installation
To install this tool, you need to have Rust installed on your machine. If you don't have Rust installed, you can download it from [here](https://www.rust-lang.org/tools/install).

Once you have Rust installed, you can clone this repository and build the project:

```bash
git clone https://github.com/fzhn/converter.git
cd converter
cargo build --release
```
The executable will be in the `./target/release` directory.

## Usage
To use the converter tool, you can run the executable followed by the value you want to convert:
```bash
./converter hex 10
```
This will convert the decimal number 10 into hexadecimal.

## Contributing
Contributions are welcome! Please feel free to submit a Pull Request.

## License
This project is licensed under the MIT License.
