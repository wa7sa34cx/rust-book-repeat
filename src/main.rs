#![allow(dead_code, unused_imports)]

mod ch04_01_ownership;
mod ch04_02_references;
mod ch04_03_slices;

use ch04_01_ownership as ownership;
use ch04_02_references as references;
use ch04_03_slices as slices;

fn main() {
    // ownership::run();
    // references::run();
    slices::run();
}