#[macro_use]
extern crate clickr_proc_macro;

#[command(
    name = "My CLI",
    about = "Does CLI Things",
    version = "0.1.0",
    author = "kszela",
    after_help = "blep"
)]
fn main() {
    println!("Hi");
}
