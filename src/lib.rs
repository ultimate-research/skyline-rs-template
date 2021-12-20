#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]
#![feature(asm)]

mod mario;
mod falco;
mod custom;

#[skyline::main(name = "smashline_test")]
pub fn main() {
    mario::install();
    falco::install();
    custom::install();
}