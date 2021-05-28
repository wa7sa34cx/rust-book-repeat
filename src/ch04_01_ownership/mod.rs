pub fn run() {
    // Mutable string
    let mut s = String::from("Hello, ");
    s.push_str("World!");
    println!("{}", s);

    // Copy
    let x = 5;
    let y = x; // value Copied here
    assert_eq!(x, y);

    // Move
    let s1 = String::from("hello");
    let s2 = s1; // value MOVED here
    // assert_eq!(s1, s2);

    // Clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); // value Cloned here
    assert_eq!(s1, s2);

    println!();

    /*
    * Ownership and Functions
    */

    let s = String::from("party 🎉");
    takes_ownership(s);
    // println!("{}", s); // borrow of moved value: `s`

    let x = 64;
    makes_copy(x);
    println!("{}", x);

    let s = gives_ownership();
    println!("{}", s);

    let s1 = String::from("Wow 😆");
    let s2 = takes_and_gives_back(s1);
    println!("{}", s2);

}

fn takes_ownership(value: String) {
  println!("{}", value);
}

fn makes_copy(value: i32) {
  println!("{}", value);
}

fn gives_ownership() -> String {
  let s = String::from("Beach 🏖");
  s
}

fn takes_and_gives_back(string: String) -> String {
  string
}