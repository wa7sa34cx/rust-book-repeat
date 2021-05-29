pub fn run() {
    println!("Learning vectors in Rust 🦀");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);
}