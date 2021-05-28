pub fn run() {
    let s = String::from("I'm goning to the party 🎉");

    // println!("{}", first_word(&s));
    // first_word(&s);
    // println!("{}", b' ');

    // let first = first_word(&s[..]);
    println!("{}", first_word(&s[..]));

    // s.clear();
    // println!("{}", first);

    // String literal
    let lit = "Hello world";
    println!("{}", first_word(lit));

    /*
    * Refer to part of an array
    */
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[..3];
    println!("{:?}", slice);


}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &val) in bytes.iter().enumerate() {
        if val == b' ' {
          return &s[..i];
        }
    }

    &s[..]
}