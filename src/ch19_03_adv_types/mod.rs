type Kilometers = i32;

type Thunk = Box<dyn Fn() + Send + 'static>;

pub fn run() {
    let x = 5;
    let y: Kilometers = 6;
    println!("x + y = {}", x + y);

    let f: Thunk = Box::new(|| println!("hi"));
}