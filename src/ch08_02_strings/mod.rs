pub fn run() {
    let mut s = String::from("foo");
    let s2 = "bar";
    s.push_str(s2);
    println!("s is `{}`", s);
    println!("s2 is `{}`", s2);

    let s3 = s + s2;
    println!("s3 is `{}`", s3);
    // println!("s is `{}`", s);

    println!();

    let a = "poop";
    let b = "boop";
    let c = String::with_capacity(10) + a + b;

    println!("c is `{}`", c);
    println!("lengh of c is {}", c.len());
    println!("capasity of c is {}", c.capacity());

    println!();

    let hello = "лох";
    println!("{:?}", &hello[..]);
    println!("{:?}", &hello[0..2]);

    let chars = hello.chars();
    for c in chars {
        println!("{}", c);
    }

    let bytes = hello.bytes();
    for b in bytes {
        println!("{}", b);
    }

}