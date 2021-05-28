pub fn run() {
    let user = User {
        username: String::from("Bob"),
        email: String::from("bob@amazon.com"),
        sign_in_count: 5,
        active: true,
    };

    println!("{:#?}", user);

    // ---
    println!();

    let mut user = User {
        username: String::from("Bob"),
        email: String::from("bob@amazon.com"),
        sign_in_count: 5,
        active: true,
    };

    user.username = String::from("Mira");

    println!("{:#?}", user);

    // ---
    println!();

    let user = create_user(String::from("Alice"), String::from("alice@me.com"));
    println!("{:#?}", user);

    let user2 = User {
        username: String::from("Mari"),
        email: String::from("mari@me.com"),
        // sign_in_count: user.sign_in_count,
        // active: user.active,
        ..user
    };
    println!("{:#?}", user2);

    /*
    * Using Tuple Structs
    */ 
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    /*
    * Rectangle and its area
    */
    let mut rect = Rectangle {
      width: 60,
      height: 30,
    };
    println!("{:#?}", rect);
    println!("The area of rectangle is {}", rect.area());

    rect.width(70);
    println!("The area of new rectangle is {}", rect.area());

    /*
    * Square
    */
    let square = Rectangle::square(50);
    println!("The area of square is {}", square.area());

}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn create_user(username: String, email: String) -> User {
    User {
      username,
      email,
      sign_in_count: 1,
      active: true,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
      self.width * self.height
    }
}

impl Rectangle {
    fn width(&mut self, width: u32) {
        self.width = width;
    }
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}