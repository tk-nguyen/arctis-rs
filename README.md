# arctis-rs
A small utility to query your Arctis device battery level, written in Rust.

The SteelSeries Engine does not show the battery level in numbers, only the icon so I wrote this to get more details. Also to practice my Rust 😀.

## Features
Only getting battery level for now. Maybe I'll add more features in the future? ¯\\_(ツ)\_/¯​ 

## Downloads
Head to the release. At the moment only Windows binaries are available.

## Usage
Run the program from a terminal window.
Here's the help options:

```bash
arctis-rs 0.1.1
A small utility to query Arctis battery

USAGE:
    arctis-rs.exe [FLAGS]

FLAGS:
    -b, --battery    Query the battery level
    -h, --help       Prints help information
    -l, --list       Get the device list
    -V, --version    Prints version information
```

## Contribute 
If you want to contribute to this, it's really simple.

- First, download rust from https://rustup.rs/ (If you're on Windows you have to install the VS Build Tools too, the installer will tell you how).

- Then, clone this repository and build with `cargo`: `cargo build`.

- Run the program with `cargo run`.

``` bash
git clone https://github.com/tk-nguyen/arctis-rs
cd arctis-rs/
cargo build && cargo run
```

If you have any problem, please report it in the issues! I'll try to get back to you as soon as possible.

