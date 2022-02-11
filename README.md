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

A really long string that will only print every `1000` generation:
```sh
$ ./stringgen "Sed ut perspiciatis unde omnis iste natus error sit voluptatem accusantium doloremque laudantium, totam rem aperiam, eaque ipsa quae ab illo inventore veritatis et quasi architecto beatae vitae dicta sunt explicabo. Nemo enim ipsam voluptatem quia voluptas sit aspernatur aut odit aut fugit, sed quia consequuntur magni dolores eos qui ratione voluptatem sequi nesciunt. Neque porro quisquam est, qui dolorem ipsum quia dolor sit amet, consectetur, adipisci velit, sed quia non numquam eius modi tempora incidunt ut labore et dolore magnam aliquam quaerat voluptatem. Ut enim ad minima veniam, quis nostrum exercitationem ullam corporis suscipit laboriosam, nisi ut aliquid ex ea commodi consequatur? Quis autem vel eum iure reprehenderit qui in ea voluptate velit esse quam nihil molestiae consequatur, vel illum qui dolorem eum fugiat quo voluptas nulla pariatur?" 70 1000
``` 

# Building

Prebuilt binaries are available for Linux, macOS and Windows in the `bin` folder.

Install the [Rust toolchain](https://www.rust-lang.org/tools/install).

Open the project directory (the one that contains Cargo.toml) and run:
```sh
$ cargo build --release
```
The executable will be located in the `target/release` directory.