pub fn run() {
    let counter = Counter::new();
    // counter.count = 10;

    for item in counter {
        println!("{}", item);
    }

    let counter = Counter::new();
    let sum: u32 = counter.sum();
    println!("Sum is {}", sum);
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < 5 {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}
