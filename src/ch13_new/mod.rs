pub fn run() {
    let x = Box::new(5);

    println!("{}", x);
    println!("{}", x);

    let closure = move |m| m + *x;

    println!("{}", closure(5));
    println!("{}", closure(7));
    // println!("{}", x);

    let v1 = vec![1, 2, 3];
    
    let v2 = v1.iter().map(|i| i + 1).collect::<Vec<i32>>();
    println!("{:?}", v2);

    // (0..5).map(|x| println!("{}", x));

    for i in 1..=5 {
        println!("{}. -", i);
    }

    let mut v = vec![1, 2, 3].into_iter();
    // let v = v.into_iter();

    for _ in 0..5 {
        println!("{:?}", v.next());
    }

    let v = [1, 2, 3];

    for i in v.iter() {
        println!("{:?}", i);
    }
}
