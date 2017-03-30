#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
mod binding;
extern crate libc;
use binding::sodium_init;
use binding::randombytes_uniform;

fn main() {
    println!("Hello");
    unsafe {
        sodium_init();
    };

    println!("{}", unsafe {randombytes_uniform(123456)});
}
