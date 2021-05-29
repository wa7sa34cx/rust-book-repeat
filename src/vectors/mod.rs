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

    // ---

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row {
        println!("{:?}", i);
    }

    let vec = vec![0; 5];
    assert_eq!(vec, [0, 0, 0, 0, 0]);

    // popped from the vec
    let mut vec = vec![1, 2, 3, 4, 5];
    while let Some(top) = vec.pop() {
        println!("{}", top);
    }

    /*
    * Implementations
    */

    println!("\n\rImplementation\n\r");

    /*
    * new
    *
    * Constructs a new, empty Vec<T>.
    */
    let mut vec: Vec<i32> = Vec::new();
    // vec[1] = 5; Panic! Out of bond
    // vec[6] = 9;
    vec.push(1);
    vec.push(2);
    println!("{:?}", vec);

    /*
    * with_capacity
    *
    * Constructs a new, empty Vec<T>.
    */



}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}