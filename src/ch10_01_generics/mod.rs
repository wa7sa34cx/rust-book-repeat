struct Point<T> {
    x: T,
    y: T,
}

impl Point<f64> {
    fn sum(&self) -> f64 {
        self.x + self.y
    }
}

pub fn run() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };

    println!("{}", float.sum());
}
