# String Gen Alexis LOURS

## Usage

```sh
$ ./stringgen <ASCII string to match> <Population size> [Number of iterations to skip]
```

Example:

To find the string `Alain LIORET` with a population of size `70`:
```sh
$ ./stringgen "Alain LIORET" 70
```

To do the same but only print every `10` iteration:
```sh
$ ./stringgen "Alain LIORET" 70 10
```

# Building

Prebuilt binaries are available for Linux, macOS and Windows in the `bin` folder.

Install the [Rust toolchain](https://www.rust-lang.org/tools/install).

Open the project directory (the one that contains Cargo.toml) and run:
```sh
$ cargo build --release
```
The executable will be located in the `target/release` directory.