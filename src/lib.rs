#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros
)]

mod mario;
mod falco;
mod custom;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    mario::install();
    falco::install();
    custom::install();
}