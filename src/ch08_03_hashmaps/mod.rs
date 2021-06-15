use std::collections::HashMap;

pub fn run() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 20);
    scores.insert(String::from("red"), 50);

    let mut my_hash = HashMap::new();
    my_hash.insert("blue", 50);
    my_hash.insert("orange", 20);

    println!("{:?}", my_hash);

    println!("The score of `blue` is {:?}", my_hash.get("blue").unwrap());

    /*
     * Implementations
     */

    // Explore this later ...
}
