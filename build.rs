use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rustc-link-lib=sodium");
}
