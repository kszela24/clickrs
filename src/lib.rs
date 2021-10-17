/*!

> Simplified CLI creation via procedural macros.

This crate is largely inspired by the `click` Python package which allows for defining command line
interfaces very simply through the use of decorators (https://github.com/pallets/click#a-simple-example).
In `clickrs`, we export a precedural macro to mimic this functionality of click: `command`.

Thanks to the fantastic work done by the `structopt` package and taking inspiration from the Python `fire`
package (https://github.com/google/python-fire#basic-usage), we have taken this a step further.  Building
`clickrs` on `structopt` allows us to leverage the argument types in the function wrapped by the `command`
procedural macro, which allows for very clean and succinct CLI definitions.

To be clear, `argument` is not a macro, it's just used as a way to provide a macro-like interface for
`command` to pick up additional options for each argument, so you won't need to import it.

Re-implementing the example CLI in the structopt documention (https://github.com/TeXitoi/structopt/blob/master/README.md#example),
with `clickrs` we can simplify it even further:

```
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

## Defaults without using `argument`

You only need to use `argument` if you need to specify additional options for the CLI inputs. As shown
below, if the defaults provided by `structopt` are what you need, we can use the defaults provided
by `structopt` by foregoing the calls to `argument`.  This makes defining a CLI even easier:

```
use clickrs::command;

#[command(
    name = "example",
    about = "An example of clickrs with defaults for arguments."
)]
fn main(
    input: String,
    file_name: Option<String>,
) {
    println!("{}, {:?}", input, file_name);
}

```

which looks like:

```bash
example 0.1.0
An example of clickrs with defaults for arguments.

USAGE:
    clickrs <input> [file-name]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <input>
    <file-name>
```

!*/

pub use clickrs_proc_macro::command;

/// Re-exports
pub use structopt;
