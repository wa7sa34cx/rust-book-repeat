pub fn run() {
    let mut hello = String::from("Hello, ");

    hello.push('w');
    hello.push_str("orld!");

    println!("{}", hello);

    // ---
    let vec = vec![240, 159, 146, 150];
    let s = String::from_utf8(vec).unwrap();
    println!("{}", s);

    let bytes = "💖".as_bytes();
    println!("{:?}", bytes);

    // ---
    let story = String::from("Once upon a time...");
    let ptr = story.as_ptr();
    let len = story.len();
    let capacity = story.capacity();
    println!("{:?} {} {}", ptr, len, capacity);
    
    // ---
    // If a String has enough capacity, adding elements to it will not re-allocate. 
    // For example, consider this program:
    println!();
    let mut s = String::new();

    println!("{}", s.capacity());

    for _ in 0..5 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    /*
    * Implementation
    */

    /*
    * with_capasity
    */
    let mut s = String::with_capacity(10);
    let cap = s.capacity();

    for _ in 0..10 {
        s.push('a');
    }
    let cap2 = s.capacity();
    println!("cap = {}", cap);
    println!("{}", s);
    println!("cap2 = {}", cap2);


}
