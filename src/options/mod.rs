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
        None => {}
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
    fn sq(x: u32) -> Option<u32> {
        Some(x * x)
    }
    fn nope(_: u32) -> Option<u32> {
        None
    }

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

    /*
     * or_else
     *
     * Returns the option if it contains a value,
     * otherwise calls f and returns the result.
     */
    fn nobody() -> Option<&'static str> {
        None
    }
    fn vikings() -> Option<&'static str> {
        Some("vikings")
    }

    assert_eq!(Some("barbarians").or_else(vikings), Some("barbarians"));
    assert_eq!(None.or_else(vikings), Some("vikings"));
    assert_eq!(None.or_else(nobody), None);

    /*
     * xor
     *
     * Returns Some if exactly one of self, optb is Some,
     * otherwise returns None.
     */
    println!();
    let a = Some(5);
    let b = Some(7);
    let c: Option<u32> = None;
    println!("{:?}", a.xor(b));
    println!("{:?}", a.xor(c));
    println!("{:?}", b.xor(c));
    println!("{:?}", c.xor(None));

    /*
     * get_or_insert
     *
     * Inserts value into the option if it is None,
     * then returns a mutable reference to the contained value.
     */
    println!();
    let mut x = None;

    let y: &mut u32 = x.get_or_insert(5);
    *y = 7;
    println!("{:?}", y);

    /*
     * get_or_insert_default
     *
     * This is a nightly-only experimental API.
     */

    /*
     * get_or_insert_with
     *
     * Inserts a value computed from f into the option if it is None,
     * then returns a mutable reference to the contained value.
     */
    println!();
    println!("get_or_insert_with --------------");

    let mut x = None;
    let y: u32 = 5;

    let z: &mut u32 = x.get_or_insert_with(|| y + 5);
    println!("{:?}", z);

    *z = 32;
    println!("{:?}", z);

    /*
     * Some experiment
     */
    println!();
    println!("double deref 🤔 --------------");

    let x = &4;
    let y = &x;
    println!("{:?}", *(*y));

    /*
     * take
     *
     * Takes the value out of the option, leaving a None in its place.
     */
    let mut x = Some(2);
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, Some(2));

    let mut x: Option<u32> = None;
    let y = x.take();
    assert_eq!(x, None);
    assert_eq!(y, None);

    /*
     * replace
     *
     * replace(&mut self, value: T) -> Option<T>
     *
     * Replaces the actual value in the option by the value given in parameter,
     * returning the old value if present,
     * leaving a Some in its place without deinitializing either one.
     */
    let mut x = Some(2);
    let old = x.replace(5);
    assert_eq!(x, Some(5));
    assert_eq!(old, Some(2));

    let mut x = None;
    let old = x.replace(3);
    assert_eq!(x, Some(3));
    assert_eq!(old, None);

    /*
     * zip
     *
     * Zips self with another Option.
     *
     * If self is Some(s) and other is Some(o), this method returns Some((s, o)).
     * Otherwise, None is returned.
     */
    let x = Some(1);
    let y = Some("hi");
    let z = None::<u8>;

    assert_eq!(x.zip(y), Some((1, "hi")));
    assert_eq!(x.zip(z), None);

    /*
     * copied
     *
     * Maps an Option<&T> to an Option<T> by copying the contents of the option.
     */
    let x = 12;
    let opt_x = Some(&x);
    assert_eq!(opt_x, Some(&12));
    let copied = opt_x.copied();
    assert_eq!(copied, Some(12));

    /*
     * cloned
     *
     * Maps an Option<&T> to an Option<T> by cloning the contents of the option.
     */
    let x = 12;
    let opt_x = Some(&x);
    assert_eq!(opt_x, Some(&12));
    let cloned = opt_x.cloned();
    assert_eq!(cloned, Some(12));

    /*
     * unwrap_or_default
     *
     * Returns the contained Some value or a default
     */
    let good_year_from_input = "1909";
    let bad_year_from_input = "190blarg";
    let good_year = good_year_from_input.parse().ok().unwrap_or_default();
    let bad_year = bad_year_from_input.parse().ok().unwrap_or_default();

    assert_eq!(1909, good_year);
    assert_eq!(0, bad_year);

    /*
     * as_deref
     *
     * Converts from Option<T> (or &Option<T>) to Option<&T::Target>.
     */
    let x: Option<String> = Some("hey".to_owned());
    assert_eq!(x.as_deref(), Some("hey"));

    let x: Option<String> = None;
    assert_eq!(x.as_deref(), None);

    /*
     * flatten
     *
     * Converts from Option<Option<T>> to Option<T>
     */
    let x: Option<Option<u32>> = Some(Some(6));
    assert_eq!(Some(6), x.flatten());

    let x: Option<Option<u32>> = Some(None);
    assert_eq!(None, x.flatten());

    let x: Option<Option<u32>> = None;
    assert_eq!(None, x.flatten());

    // Flattening only removes one level of nesting at a time:
    let x: Option<Option<Option<u32>>> = Some(Some(Some(6)));
    assert_eq!(Some(Some(6)), x.flatten());
    assert_eq!(Some(6), x.flatten().flatten());
}
