// Crate: The smallest amount of code that can be compiled at a t ime. It can either be a binary
// that must have a main function so it can be executed, or a library that proved functionality to
// be used by other crates. The crate root is a source file that the compiler starts from and is
// the root module of the crate. This is usually src/main.rs for binary crates and src/lib.rs for
// library crates.
// Package: A bundle of one or more crates that provide a set of functionality. Must contain a
// Cargo.toml file that describes how to build the crates. Can contain any number of binary crates
// (usually in src/bin) but only up to one library crate
//
fn main() {
    println!("Hello, world!");
}
