struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn run() {
    let my_closure = |num| {
        println!("Calculating...");
        num
    };

    println!("Do {} steps today!", my_closure(5));

    let mut closure_cache = Cacher::new(|num| {
        println!("Calculating...");
        num
    });
    
    println!("Do {} steps today!", closure_cache.value(6));
    println!("Do {} steps today!", closure_cache.value(7));
}