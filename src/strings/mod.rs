use std::char;

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

    /*
    * from_utf8_lossy
    */
    let input = b"Hello \xF0\x90\x80World";
    let output = String::from_utf8_lossy(input);
    println!("{}", output);

    /*
    * into_bytes
    */
    let s = String::from("hello");
    let bytes = s.into_bytes();
    println!("{:?}", bytes);

    let s = String::from("хелло");
    let bytes = s.into_bytes();
    println!("{:?}", bytes);

    /*
    * as_str
    */
    let s = String::from("foo");
    assert_eq!("foo", s.as_str());

    /*
    * as_mut_str
    */
    let mut s = String::from("foobar");
    let s_mut_str = s.as_mut_str();
    s_mut_str.make_ascii_uppercase();
    assert_eq!("FOOBAR", s_mut_str);

    /*
    * push_str
    */
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("{}", s);

    /*
    * reserve
    *
    * Ensures that this String’s capacity is at least additional 
    * bytes larger than its length.
    */
    let mut s = String::from('a');
    s.reserve(10);
    println!("{}", s.capacity());

    /*
    * shrink_to_fit
    *
    * Shrinks the capacity of this String to match its length.
    */
    let mut s = String::with_capacity(100);
    s.push_str("abc");
    println!("Capacity of `s` is {}", s.capacity());
    s.shrink_to_fit();
    println!("Capacity of `s` is {}", s.capacity());

    /*
    * push
    *
    * Appends the given char to the end of this String.
    */
    let mut s = String::from("abc");

    for i in 1..=5 {
        // s.push(char::from_u32(i as u32).unwrap_or('b'));
        // s.push(i.as_bytes());
        // s.push('1');
        s.push(char::from_digit(i, 10).unwrap());
    }
    println!("{}", s);
}
