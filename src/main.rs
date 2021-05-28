#![allow(dead_code, unused_imports)]

mod ch04_01_ownership;
mod ch04_02_references;
mod ch04_03_slices;
mod ch05_structs;

use ch04_01_ownership as ownership;
use ch04_02_references as references;
use ch05_structs as structs;

fn main() {
    // ownership::run();
    // references::run();
    // slices::run();
    structs::run();
}