use std::fs::File;
use std::io;
use std::io::Read;
use std::fs;

pub fn run() {
    println!("Reading file into string...");

    let s = read_from_file("hello.txt");
    println!("{:?}", s);

}

fn read_from_file(filename: &str) -> Result<String, io::Error> {
    let mut f = File::open(filename)?;

    let mut s = String::new();
    f.read_to_string(&mut s)?;

    Ok(s)
}

