#[derive(Debug)]
struct Foo {
    a: i32,
}

impl Foo {
    pub fn new() -> Self {
        Self {
            a: 0,
        }
    }

    pub fn iter(&self) -> Iter<'_> {
        Iter { inner: self, count: 0 }
    }
}

#[derive(Debug)]
pub struct Iter<'a> {
    inner: &'a Foo,
    count: i32,
}

impl Iterator for Iter<'_> {
    type Item = i32;

    // next() is the only required method
    fn next(&mut self) -> Option<Self::Item> {
        // Increment our count. This is why we started at zero.
        self.count += 1;

        // Check to see if we've finished counting or not.
        if self.count < 6 {
            Some(self.inner.a + self.count)
        } else {
            None
        }
    }
}

pub fn run() {
    let foo = Foo::new();

    for i in foo.iter() {
        println!("{:?}", i);
    }

    println!("{:?}", foo);
}