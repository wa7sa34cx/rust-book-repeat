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

    /*
     * as_bytes
     *
     * Returns a byte slice of this String’s contents.
     */
    let s = String::from("hello");
    println!("{:?}", s.as_bytes());

    /*
     * truncate
     *
     * Shortens this String to the specified length.
     */
    let mut s = String::from("hooch");
    s.truncate(2);
    println!("{}", s);

    /*
     * pop
     *
     * Removes the last character from the string buffer and returns it.
     */
    let mut s = String::from("foo");

    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('o'));
    assert_eq!(s.pop(), Some('f'));

    assert_eq!(s.pop(), None);

    /*
     * remove
     *
     * Removes a char from this String at a byte position and returns it.
     */
    println!();
    // let mut s = String::from("foo");
    let mut s = String::from("фуу");
    let bytes = s.as_bytes();
    println!("{:?}", bytes);
    let f = s.remove(0);
    println!("{}, {}", s, f);

    /*
     * retain
     *
     * Retains only the characters specified by the predicate.
     */
    let mut s = String::from("f_o_ob_ar");
    s.retain(|c| c != '_');
    println!("{}", s);

    /*
     * insert
     *
     * Inserts a character into this String at a byte position.
     */
    let mut s = String::from("foo");
    s.insert(0, 'k');
    println!("{}", s);

    /*
     * insert_str
     *
     * Inserts a string slice into this String at a byte position.
     */
    let mut s = String::from("bar");
    s.insert_str(0, "foo");
    println!("{}", s);

    /*
     * len
     *
     * Returns the length of this String, in bytes, not chars or graphemes.
     */
    println!();
    let s = String::from("foo");
    println!("The lenght of `foo` is {}", s.len());
    let s = String::from("фуу");
    println!("The lenght of `фуу` is {}", s.len());
    let s = String::from("🥁");
    println!("The lenght of `🥁` is {}", s.len());

    /*
     * is_empty
     *
     * Returns true if this String has a length of zero, and false otherwise.
     */
    let mut v = String::new();
    assert!(v.is_empty());

    v.push('a');
    assert!(!v.is_empty());

    /*
     * split_off
     *
     * Splits the string into two at the given byte index.
     */
    let mut hello = String::from("Hello, World!");
    let world = hello.split_off(7);
    assert_eq!(hello, "Hello, ");
    assert_eq!(world, "World!");

    /*
     * clear
     *
     * Truncates this String, removing all contents.
     */
    let mut s = String::from("foo");

    s.clear();

    assert!(s.is_empty());
    assert_eq!(0, s.len());
    assert_eq!(3, s.capacity());

    /*
     * drain
     *
     * Creates a draining iterator that removes the specified range in the String
     * and yields the removed chars.
     */
    let mut s = String::from("α is alpha, β is beta");
    let beta_offset = s.find('β').unwrap_or(s.len());

    // Remove the range up until the β from the string
    let t: String = s.drain(..beta_offset).collect();
    println!("{}", t);

    /*
     * replace_range
     *
     * Removes the specified range in the string, and replaces it with the given string.
     * The given string doesn’t need to be the same length as the range.
     */
    let mut s = String::from("α is alpha, β is beta");
    let beta_offset = s.find('β').unwrap_or(s.len());

    // Replace the range up until the β from the string
    s.replace_range(..beta_offset, "Α is capital alpha; ");
    assert_eq!(s, "Α is capital alpha; β is beta");

    /*
     * into_boxed_str
     *
     * Converts this String into a Box<str>.
     */
    let s = String::from("hello");
    let b = s.into_boxed_str();
    println!("{:?}", b);
}
