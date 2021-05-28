pub fn run() {
    let s = String::from("hello");
    let length = calc_length(&s);
    println!("The length of `{}` is {}", s, length);

    let length = s.len();
    println!("The length of `{}` is {}", s, length);

    // So what happens if we try to modify something we’re borrowing?
    // It doesn’t work!
    // change(&s);

    let mut s = String::from("It's a party");
    change(&mut s);
    println!("{}", s);

    /*
    * We also cannot have a mutable reference while we have an immutable one
    */
    let s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);
    println!("{} and {}", r1, r2);

    /*
    * Dangling References
    */ 
    // let ref_to_nothing = dangle();

}

fn calc_length(s: &String) -> usize {
  s.len()
}

// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

fn change(some_string: &mut String) {
    some_string.push_str(" time 🎉");
}

// fn dangle() -> &String {
//   let s = String::from("hello");
//   &s
// }