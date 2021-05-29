pub fn run() {
    let mut rectangle = Rectangle::new(15.2, 2.7);
    println!("The area of rectangle is {}", rectangle.area());

    let circle = Circle { radius: 10.0 };
    println!("The area of circle is {}", circle.area());

    // let shape = rectangle;

    // match shape {
    //     Rectangle { lenght, .. } => println!("This is rectangle with lenght = {}", lenght),
    //     Circle => println!("This is circle"),
    // }

    // rectangle.reshape(10.0, 3.4);
    rectangle.height = 3.0;
    println!("The area of reshaped rectangle is {}", rectangle.area());

    let shape = Shape::Rectangle(rectangle);
    shape.draw();
}

#[derive(Debug)]
enum Shape {
    Rectangle(Rectangle),
    Circle(Circle),
}

impl Shape {
    fn draw(&self) {
        println!("This is {:?}", self);
    }
}

#[derive(Debug)]
struct Rectangle {
    lenght: f64,
    height: f64,
}

impl Rectangle {
    fn new(lenght: f64, height: f64) -> Rectangle {
        Rectangle { lenght, height }
    }

    fn reshape(&mut self, lenght: f64, height: f64) {
        self.lenght = lenght;
        self.height = height;
    }

    fn area(&self) -> f64 {
        self.lenght * self.height
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}

impl Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}