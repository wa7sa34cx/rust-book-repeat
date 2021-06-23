use std::ops::Deref;

pub fn run() {
    let b = Box::new(5);
    println!("b = {}", b);

    // let list = Cons(1, Cons(2, Cons(3, Nil)));

    let x = 5;
    let y = Box::new(x);
    assert_eq!(x, *y);

    // ----

    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(x, *y);

    // ----

    let name = MyBox::new(String::from("Mark"));
    hello(&name);
}

enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Some dropping here");
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}