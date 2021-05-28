#![allow(dead_code, unused_imports)]

mod ch04_01_ownership;
mod ch04_02_references;

use ch04_01_ownership as ownership;
use ch04_02_references as references;

fn main() {
    // ownership::run();
    references::run();
    // color::run();
}