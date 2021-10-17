# Click-rs

A Rust clone of Python's [click](https://github.com/pallets/click), built on the [structopt](https://github.com/TeXitoi/structopt) crate.

## Documentation

Find it on [Docs.rs](https://docs.rs/clickrs).

## Example

Add `clickrs` to your dependencies of your `Cargo.toml`:
```toml
[dependencies]
clickrs = "0.1"
```

And then, in your rust file:
```rust
use clickrs::command;
use std::path::PathBuf;

#[command(name = "basic")]
#[argument("debug", short, long)]
#[argument("verbose", short, long, parse(from_occurrences))]
#[argument("speed", short, long, default_value = "42")]
#[argument("output", short, long, parse(from_os_str))]
#[argument("nb_cars", short = "c", long)]
#[argument("level", short, long)]
#[argument("files", name = "FILE", parse(from_os_str))]
fn main(
    debug: bool,
    verbose: u8,
    speed: f64,
    output: PathBuf,
    nb_cars: Option<i32>,
    level: Vec<String>,
    files: Vec<PathBuf>,
) {
    println!("{:?}", speed);
}
```

Using this example:
```
$ ./basic
error: The following required arguments were not provided:
    --output <output>

USAGE:
    clickrs --output <output> --speed <speed>

For more information try --help
```

```
$ ./basic --help
basic 0.1.0

USAGE:
    clickrs [FLAGS] [OPTIONS] --output <output> [--] [FILE]...

FLAGS:
    -d, --debug
    -h, --help       Prints help information
    -V, --version    Prints version information
    -v, --verbose

OPTIONS:
    -l, --level <level>...
    -c, --nb-cars <nb-cars>
    -o, --output <output>
    -s, --speed <speed>         [default: 42]

ARGS:
    <FILE>...
```

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.
