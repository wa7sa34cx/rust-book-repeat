pub fn run() {
    let x = 5;
    println!("do twice = {}", do_twice(add_one, x));
}

fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}