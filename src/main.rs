#[macro_use]
extern crate clickrs_proc_macro;

#[command(
    name = "My CLI",
    about = "Does CLI Things",
    version = "0.1.0",
    author = "kszela"
)]
fn main(
    arg: String,
    other_arg: String,
) {
    println!("{}", arg);
    println!("{}", other_arg);
}
