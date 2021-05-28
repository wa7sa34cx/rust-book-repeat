pub fn run() {
    println!("Options! 🤓");
    println!("https://doc.rust-lang.org/std/option/enum.Option.html");

    /*
    * is_some
    * 
    * Returns true if the option is a Some value.
    */
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_some(), true);

    if x.is_some() {
        println!("x is a Some value");
    }

    /*
    * is_none
    */
    let x: Option<u32> = None;

    if x.is_none() {
        println!("x is a None value");
    }

    /*
    * as_ref
    */
    let text: Option<String> = Some("Hello, world!".to_string());
    // let text: Option<String> = None;
    // First, cast `Option<String>` to `Option<&String>` with `as_ref`,
    // then consume *that* with `map`, leaving `text` on the stack.
    let text_length: Option<usize> = text.as_ref().map(|s| s.len());
    println!("Still can print text: {:?}", text);

    /*
    * expect
    *
    * Returns the contained Some value, consuming the self value.
    * Panics if the value is a None with a custom panic message provided by msg.
    */
    let x: Option<&str> = None;
    // x.expect("fruits are healthy");

    /*
    * unwrap
    *
    * Returns the contained Some value, consuming the self value.
    */
    let x = Some("air");
    let y = x.unwrap();
    println!("{}", y);

    /*
    * unwrap_or
    *
    * Returns the contained Some value or a provided default.
    */
    let x = Some("car").unwrap_or("bike");
    println!("{}", x);
    let y = None.unwrap_or("bike");
    println!("{}", y);

    /*
    * unwrap_or_else
    *
    * Returns the contained Some value or computes it from a closure.
    */
    let k = 10;
    let a = Some(5).unwrap_or_else(|| k * 2);
    println!("a = {}", a);
    let b = None.unwrap_or_else(|| k * 2);
    println!("b = {}", b);

    /*
    * map
    *
    * Maps an Option<T> to Option<U> by applying 
    * a function to a contained value.
    */
    let s = Some(String::from("hello"));
    let lenght = s.map(|s| s.len());
    println!("lenght: {:?}", lenght);

    /*
    * map_or
    *
    * Applies a function to the contained value (if any), 
    * or returns the provided default (if not).
    * 42 - default
    */
    let x = Some(5);
    let x = x.map_or(42, |x| x * 5);
    println!("{}", x);
    // let y: Option<i32> = None;
    let y = None.map_or(42, |v: i32| v * 5);
    println!("{}", y);

    /*
    * map_or_else
    *
    * Applies a function to the contained value (if any), 
    * or computes a default (if not).
    */
    let k = 2;

    let x = Some(5);
    let x = x.map_or_else(|| k * 2, |v| v * 3);
    println!("{}", x);

    let y: Option<i32> = None;
    let y = y.map_or_else(|| k * 2, |v| v * 3);
    println!("{}", y);

    /*
    * ok_or
    *
    * Transforms the Option<T> into a Result<T, E>, 
    * mapping Some(v) to Ok(v) and None to Err(err).
    */
    let x = Some("foo");
    let x = x.ok_or("some error");
    println!("{:?}", x);

    let y: Option<&str> = None;
    let y = y.ok_or("some error");
    println!("{:?}", y);

    /*
    * ok_or_else
    *
    * Transforms the Option<T> into a Result<T, E>, 
    * mapping Some(v) to Ok(v) and None to Err(err()).
    */
    let x = Some("foo");
    let x = x.ok_or_else(|| "some error");
    println!("{:?}", x);

    let y: Option<&str> = None;
    let y = y.ok_or_else(|| "some error");
    println!("{:?}", y);

    /*
    * iter
    *
    * Returns an iterator over the possibly contained value.
    */
    let x = Some(4_u32);
    let x = x.iter().next();
    println!("{:?}", x);

    let y: Option<u32> = None;
    let y = y.iter().next();
    println!("{:?}", y);

    /*
    * iter_mut
    *
    * Returns a mutable iterator over the possibly contained value.
    */
    let mut x = Some(4);
    match x.iter_mut().next() {
        Some(v) => *v = 42,
        None => {},
    }
    assert_eq!(x, Some(42));

    let mut x: Option<u32> = None;
    assert_eq!(x.iter_mut().next(), None);

    /*
    * and
    */
    let x = Some(2);
    let y: Option<&str> = None;
    assert_eq!(x.and(y), None);

    let x: Option<u32> = None;
    let y = Some("foo");
    assert_eq!(x.and(y), None);

    let x = Some(2);
    let y = Some("foo");
    assert_eq!(x.and(y), Some("foo"));

    let x: Option<u32> = None;
    let y: Option<&str> = None;
    assert_eq!(x.and(y), None);

    /*
    * and_then
    *
    * Returns None if the option is None,
    * otherwise calls f with the wrapped value and returns the result.
    */
    fn sq(x: u32) -> Option<u32> { Some(x * x) }
    fn nope(_: u32) -> Option<u32> { None }

    assert_eq!(Some(2).and_then(sq).and_then(sq), Some(16));
    assert_eq!(Some(2).and_then(sq).and_then(nope), None);
    assert_eq!(Some(2).and_then(nope).and_then(sq), None);
    assert_eq!(None.and_then(sq).and_then(sq), None);

    /*
    * filter
    *
    * Returns None if the option is None, 
    * otherwise calls predicate with the wrapped value and returns:
    *
    * Some(t) if predicate returns true (where t is the wrapped value), and
    * None if predicate returns false.
    */
    fn is_even(n: &i32) -> bool {
        n % 2 == 0
    }
    assert_eq!(None.filter(is_even), None);
    assert_eq!(Some(3).filter(is_even), None);
    assert_eq!(Some(4).filter(is_even), Some(4));

    /*
    * or
    *
    * Returns the option if it contains a value, 
    * otherwise returns optb.
    */
    let x = Some(2);
    let y = None;
    assert_eq!(x.or(y), Some(2));

    let x = None;
    let y = Some(100);
    assert_eq!(x.or(y), Some(100));

    let x = Some(2);
    let y = Some(100);
    assert_eq!(x.or(y), Some(2));

    let x: Option<u32> = None;
    let y = None;
    assert_eq!(x.or(y), None);

}