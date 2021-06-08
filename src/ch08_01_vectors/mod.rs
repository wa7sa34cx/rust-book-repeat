pub fn run() {
    let mut vec = vec![1, 2, 3];
    for i in &mut vec {
        *i *= 10;
    }
    println!("{:?}", vec);

    let vec: Vec<i32> = vec.iter().map(|x| x * 10).collect();
    println!("{:?}", vec);
}
