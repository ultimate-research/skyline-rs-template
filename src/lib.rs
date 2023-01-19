#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    unused_macros,
    clippy::borrow_interior_mutable_const
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