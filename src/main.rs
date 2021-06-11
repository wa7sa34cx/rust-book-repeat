#![allow(dead_code, unused_imports)]

mod ch04_01_ownership;
mod ch04_02_references;
mod ch04_03_slices;
mod ch05_structs;
mod ch06_enums;
mod ch08_01_vectors;
mod ch08_02_strings;
mod ch08_03_hashmaps;
mod iterator;
mod options;
mod primitive_i32;
mod shapes;
mod strings;
mod strs;
mod vectors;
mod ch09_errors;

use ch04_01_ownership as ownership;
use ch04_02_references as references;
use ch05_structs as structs;
use ch06_enums as enums;
use ch08_01_vectors as vectors_learning;
use ch08_02_strings as strings_learning;
use ch08_03_hashmaps as hashmaps;
use ch09_errors as errors;

fn main() {
    // ownership::run();
    // references::run();
    // slices::run();
    // structs::run();
    // enums::run();
    // options::run();
    // shapes::run();
    // vectors::run();
    // strings::run();
    // primitive_i32::run();
    // iterator::run();
    // strs::run();
    // vectors_learning::run();
    // strings_learning::run();
    // hashmaps::run();
    errors::read_to_string();
}
