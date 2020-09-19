#![feature(concat_idents)]
#![feature(proc_macro_hygiene)]

mod mario;
mod falco;
mod custom;

#[skyline::main(name = "acmd_test")]
pub fn main() {
    mario::install();
    falco::install();
    custom::install();
}