pub fn run() {
    println!("Is Rust an OOP language?");

    let mut my = MyStruct::new();
    my.add(10);
    my.add(20);
    my.add(30);

    println!("The average is {}", my.average());

    let temp = my.average;
    println!("Can I get an average: {}", temp);

}

struct MyStruct {
    list: Vec<i32>,
    average: f64,
}

impl MyStruct {
    pub fn new() -> MyStruct {
        MyStruct {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let sum: i32 = self.list.iter().sum();
        self.average = sum as f64 / self.list.len() as f64;
    }

}