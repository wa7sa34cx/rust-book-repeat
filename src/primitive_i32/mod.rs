pub fn run() {
    assert_eq!(i32::MIN, -2147483648);

    /*
    * is_negative
    *
    * Returns true if self is negative and false if the number is zero or positive.
    */
    assert!((-10i32).is_negative());
    assert!(!10i32.is_negative());

    /*
    * is_pozitive
    *
    * Returns true if self is pozitive and false if the number is zero or negative.
    */
    assert!(10i32.is_positive());
    assert!(!(-10i32).is_positive());

    /*
    * abs
    *
    * Computes the absolute value of self.
    */
    let i: i32 = -5;
    println!("{}", i.abs());
}