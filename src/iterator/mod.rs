pub fn run() {
    let a = [1, 2, 3, 4, 5];

    let iter = a.iter();

    // println!("{:?}", iter.next());
    // assert_eq!(Some(&1), iter.next());

    for item in iter {
        println!("{:?}", item);
    }
    println!("{:?}", a);

    println!();

    /*
    * count
    *
    * Consumes the iterator, counting the number of iterations and returning it.
    */
    let a = [1, 2, 3];
    println!("{}", a.iter().count());
    println!("{:?}", a);

    println!();

    /*
    * last
    *
    * Consumes the iterator, returning the last element.
    */
    let a = [1, 2, 3];
    println!("{:?}", a.iter().last());
    
    let b: Vec<i32> = Vec::new();
    println!("{:?}", b.iter().last());

    println!();
    
    /*
    * nth
    *
    * Returns the nth element of the iterator.
    */
    let a = [1, 2, 3];
    println!("{:?}", a.iter().nth(1));
    println!("{:?}", a.iter().nth(10));
    println!();

    


}