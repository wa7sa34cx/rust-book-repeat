use std::cmp::Ordering;

pub(crate) fn run() {
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

    /*
     * step_by
     *
     * Creates an iterator starting at the same point,
     * but stepping by the given amount at each iteration.
     */
    let a = [0, 1, 2, 3, 4, 5];
    let iter = a.iter().step_by(2);
    for item in iter {
        println!("{:?}", item);
    }
    println!();

    /*
     * chain
     *
     * Takes two iterators and creates a new iterator over both in sequence.
     */
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let mut iter = a1.iter().chain(a2.iter());
    for item in iter {
        print!("{:?}-", item);
    }
    println!();
    println!();

    /*
     * zip
     *
     * ‘Zips up’ two iterators into a single iterator of pairs.
     */
    let a1 = [1, 2, 3];
    let a2 = [4, 5, 6];

    let iter = a1.iter().zip(a2.iter());
    for item in iter {
        print!("{:?}-", item);
    }
    println!();

    /*
     * map
     *
     * Takes a closure and creates an iterator
     * which calls that closure on each element.
     */
    let a = [1, 2, 3];
    let iter = a.iter().map(|x| 2 * x);
    for item in iter {
        print!("{:?}-", item);
    }
    println!();

    /*
     * filter
     *
     * Creates an iterator which uses a closure to determine if an element
     * should be yielded.
     */
    let a = [0i32, 1, 2];
    let iter = a.iter().filter(|x| x.is_positive());
    for item in iter {
        print!("{:?}-", item);
    }
    println!();

    let a = [0, 1, 2];
    let iter = a.iter().filter(|x| **x > 1); // need two *s!
    for item in iter {
        print!("{:?}-", item);
    }
    println!();

    // It’s common to instead use destructuring on the argument to strip away one:
    let a = [0, 1, 2];
    let iter = a.iter().filter(|&x| *x > 1); // both & and *
                                             // let mut iter = a.iter().filter(|&&x| x > 1); // two &s
    for item in iter {
        print!("{:?}-", item);
    }
    println!();

    /*
     * filter_map
     *
     * Creates an iterator that both filters and maps.
     */
    let a = ["1", "two", "NaN", "four", "5"];
    let mut iter = a.iter().filter_map(|s| s.parse().ok());
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(5));
    assert_eq!(iter.next(), None);
    println!();

    /*
     * enumerate
     *
     * Creates an iterator which gives the current iteration
     * count as well as the next value
     */
    let a = ['a', 'b', 'c'];
    let iter = a.iter().enumerate();
    for (i, item) in iter {
        println!("{}-{}", i, item);
    }
    println!();

    /*
     * peekable
     *
     * Creates an iterator which can use peek to look at the next element
     * of the iterator without consuming it.
     */
    let xs = [1, 2, 3];

    let mut iter = xs.iter().peekable();

    // peek() lets us see into the future
    assert_eq!(iter.peek(), Some(&&1));
    assert_eq!(iter.next(), Some(&1));

    assert_eq!(iter.next(), Some(&2));

    // we can peek() multiple times, the iterator won't advance
    assert_eq!(iter.peek(), Some(&&3));
    assert_eq!(iter.peek(), Some(&&3));

    assert_eq!(iter.next(), Some(&3));

    // after the iterator is finished, so is peek()
    assert_eq!(iter.peek(), None);
    assert_eq!(iter.next(), None);

    /*
     * skip_while
     *
     * Creates an iterator that skips elements based on a predicate.
     */
    let a = [-1i32, 0, 1, -2, -3, 5];
    let iter = a.iter().skip_while(|x| x.is_negative());
    for item in iter {
        print!("{}, ", item);
    }
    println!();

    /*
     * take_while
     *
     * Creates an iterator that yields elements based on a predicate.
     */
    let a = [-1i32, 0, 1];
    let iter = a.iter().take_while(|x| x.is_negative());
    for item in iter {
        print!("{}, ", item);
    }
    println!();

    /*
     * skip
     *
     * Creates an iterator that skips the first n elements.
     */
    let a = [1, 2, 3];
    let iter = a.iter().skip(2);
    // println!("{:?}", iter);
    for item in iter {
        print!("{}, ", item);
    }
    println!();

    /*
     * take
     *
     * Creates an iterator that yields its first n elements.
     */
    let a = [1, 2, 3];
    let iter = a.iter().take(2);
    for item in iter {
        print!("{}, ", item);
    }
    println!();

    /*
     * flat_map
     *
     * Creates an iterator that works like map, but flattens nested structure.
     */
    let words = ["alpha", "beta", "gamma"];

    // chars() returns an iterator
    // let iter = words.iter().flat_map(|s| s.chars()).collect();
    let iter = words.iter().flat_map(|s| s.chars());
    for item in iter {
        print!("{}, ", item);
    }
    println!();

    /*
     * flatten
     *
     * Creates an iterator that flattens nested structure.
     */
    let data = vec![vec![1, 2, 3, 4], vec![5, 6]];
    // let flattened = data.into_iter().flatten().collect::<Vec<u8>>();
    let flattened = data.into_iter().flatten();
    for item in flattened {
        print!("{}, ", item);
    }
    println!();

    /*
     * fuse
     *
     * Creates an iterator which ends after the first None.
     */
    // let vec = vec![Some(3), Some(5), None, Some(6)];
    // let mut iter = vec.fuse();
    // for item in iter {
    //     print!("{}, ", item);
    // }
    // println!();

    /*
     * by_ref
     *
     * Borrows an iterator, rather than consuming it.
     */
    let a = [1, 2, 3];

    let mut iter = a.iter();

    // instead, we add in a .by_ref()
    let sum: i32 = iter.by_ref().take(2).fold(0, |acc, i| acc + i);

    assert_eq!(sum, 3);

    // now this is just fine:
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.next(), None);

    /*
     * collect
     *
     * Transforms an iterator into a collection.
     */
    let a = [1, 2, 3];
    let doubled: Vec<i32> = a.iter().map(|&x| x * 2).collect();
    // let doubled = a.iter().map(|x| x * 2).collect::<Vec<i32>>();
    // let doubled = a.iter().map(|x| x * 2).collect::<Vec<_>>();
    println!("{:?}", doubled);

    let chars = ['g', 'd', 'k', 'k', 'n'];

    let hello: String = chars
        .iter()
        .map(|&x| x as u8)
        .map(|x| (x + 1) as char)
        .collect();
    println!("{}", hello);

    // ---
    let alphabet = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];
    let codes: Vec<u8> = alphabet.iter().map(|&x| x as u8).collect();
    println!("{:?}", codes);

    /*
     * partition
     *
     * Consumes an iterator, creating two collections from it.
     */
    let a = [1, 2, 3];

    let (even, odd): (Vec<i32>, Vec<i32>) = a.iter().partition(|&n| n % 2 == 0);

    println!("{:?}", even);
    println!("{:?}", odd);

    /*
     * fold
     *
     * Folds every element into an accumulator by applying an operation,
     * returning the final result.
     */
    let a = [1, 2, 3];

    // the sum of all of the elements of the array
    let sum = a.iter().fold(0, |acc, x| acc + x);

    println!("{}", sum);

    /*
     * reduce
     *
     * Reduces the elements to a single one,
     * by repeatedly applying a reducing operation.
     */
    let a = [10, 20, 5, -23, 0];
    let b: [u32; 0] = [];

    let max_a = a.iter().reduce(|a, b| if a >= b { a } else { b });
    let max_b = b.iter().reduce(|a, b| if a >= b { a } else { b });
    println!("{:?}", max_a);
    println!("{:?}", max_b);

    /*
     * all
     *
     * Tests if every element of the iterator matches a predicate.
     */
    let a = [1, 2, 3];
    assert!(a.iter().all(|&x| x > 0));
    assert!(!a.iter().all(|&x| x > 2));

    /*
     * any
     *
     * Tests if every element of the iterator matches a predicate.
     */
    let a = [1, 2, 3];
    assert!(a.iter().any(|&x| x > 0));
    assert!(a.iter().any(|&x| x > 2));

    /*
     * find
     *
     * Searches for an element of an iterator that satisfies a predicate.
     */
    let a = [1, 2, 3];
    assert_eq!(a.iter().find(|&&x| x == 2), Some(&2));
    assert_eq!(a.iter().find(|&&x| x == 5), None);

    /*
     * find_map
     *
     * Applies function to the elements of iterator
     * and returns the first non-none result.
     */
    let a = ["lol", "NaN", "2", "5"];
    let first_number = a.iter().find_map(|s| s.parse().ok());
    assert_eq!(first_number, Some(2));

    /*
     * position
     *
     * Searches for an element in an iterator, returning its index.
     */
    let a = [1, 2, 3];
    assert_eq!(a.iter().position(|&x| x == 2), Some(1));
    assert_eq!(a.iter().position(|&x| x == 5), None);

    /*
     * rposition
     *
     * Searches for an element in an iterator from the right, returning its index.
     */
    let a = [1, 2, 3];
    assert_eq!(a.iter().rposition(|&x| x == 3), Some(2));
    assert_eq!(a.iter().rposition(|&x| x == 5), None);

    /*
     * max
     *
     * Returns the maximum element of an iterator.
     */
    let a = [1, 2, 3];
    let b: Vec<u32> = Vec::new();

    assert_eq!(a.iter().max(), Some(&3));
    assert_eq!(b.iter().max(), None);

    /*
     * min
     *
     * Returns the minimum element of an iterator.
     */
    let a = [1, 2, 3];
    let b: Vec<u32> = Vec::new();

    assert_eq!(a.iter().min(), Some(&1));
    assert_eq!(b.iter().min(), None);

    /*
     * rev
     *
     * Reverses an iterator’s direction.
     */
    println!();
    let a = [1, 2, 3, 4, 5];
    for item in a.iter().rev() {
        println!("{}", item);
    }

    /*
     * unzip
     *
     * Converts an iterator of pairs into a pair of containers.
     */
    let a = [(1, 2), (3, 4)];
    let (left, right): (Vec<_>, Vec<_>) = a.iter().cloned().unzip();

    assert_eq!(left, [1, 3]);
    assert_eq!(right, [2, 4]);

    /*
     * copied
     *
     * Converts an iterator of pairs into a pair of containers.
     */
    let a = [1, 2, 3];

    let v_copied: Vec<_> = a.iter().copied().collect();

    // copied is the same as .map(|&x| x)
    let v_map: Vec<_> = a.iter().map(|&x| x).collect();

    assert_eq!(v_copied, vec![1, 2, 3]);
    assert_eq!(v_map, vec![1, 2, 3]);

    /*
     * cloned
     *
     * Creates an iterator which clones all of its elements.
     */
    let a = [1, 2, 3];

    let v_cloned: Vec<_> = a.iter().cloned().collect();

    // cloned is the same as .map(|&x| x), for integers
    let v_map: Vec<_> = a.iter().map(|&x| x).collect();

    assert_eq!(v_cloned, vec![1, 2, 3]);
    assert_eq!(v_map, vec![1, 2, 3]);

    /*
     * cycle
     *
     * Repeats an iterator endlessly.
     */
    let a = [1, 2, 3];

    let mut it = a.iter().cycle();

    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), Some(&1));
    assert_eq!(it.next(), Some(&2));
    assert_eq!(it.next(), Some(&3));
    assert_eq!(it.next(), Some(&1));

    /*
     * sum
     *
     * Sums the elements of an iterator.
     */
    let a = [1, 2, 3];
    let sum: i32 = a.iter().sum();
    assert_eq!(sum, 6);

    /*
     * product
     *
     * Iterates over the entire iterator, multiplying all the elements
     */
    fn factorial(n: u32) -> u32 {
        (1..=n).product()
    }
    println!("{}", factorial(5));
    println!();

    /*
     * cmp
     *
     * Lexicographically compares the elements of this Iterator with those of another.
     */
    assert_eq!([1].iter().cmp([1].iter()), Ordering::Equal);
    assert_eq!([1].iter().cmp([1, 2].iter()), Ordering::Less);
    assert_eq!([1, 2].iter().cmp([1].iter()), Ordering::Greater);

    /*
     * eq
     *
     * Determines if the elements of this Iterator are equal to those of another.
     */
    assert_eq!([1].iter().eq([1].iter()), true);
    assert_eq!([1].iter().eq([1, 2].iter()), false);

    /*
     * ne
     *
     * Determines if the elements of this Iterator are not equal to those of another.
     */
    assert_eq!([1].iter().ne([1].iter()), false);
    assert_eq!([1].iter().ne([1, 2].iter()), true);

    /*
     * lt
     *
     * Determines if the elements of this Iterator are lexicographically
     * less than those of another.
     */
    assert_eq!([1].iter().lt([1].iter()), false);
    assert_eq!([1].iter().lt([1, 2].iter()), true);
    assert_eq!([1, 2].iter().lt([1].iter()), false);
    assert_eq!([1, 2].iter().lt([1, 2].iter()), false);

    /*
     * le
     *
     * Determines if the elements of this Iterator are lexicographically
     * less or equal to those of another.
     */
    assert_eq!([1].iter().le([1].iter()), true);
    assert_eq!([1].iter().le([1, 2].iter()), true);
    assert_eq!([1, 2].iter().le([1].iter()), false);
    assert_eq!([1, 2].iter().le([1, 2].iter()), true);

    /*
     * gt
     *
     * Determines if the elements of this Iterator are lexicographically
     * greater than those of another.
     */
    assert_eq!([1].iter().gt([1].iter()), false);
    assert_eq!([1].iter().gt([1, 2].iter()), false);
    assert_eq!([1, 2].iter().gt([1].iter()), true);
    assert_eq!([1, 2].iter().gt([1, 2].iter()), false);

    /*
     * ge
     *
     * Determines if the elements of this Iterator are lexicographically
     * greater or equal than those of another.
     */
    assert_eq!([1].iter().ge([1].iter()), true);
    assert_eq!([1].iter().ge([1, 2].iter()), false);
    assert_eq!([1, 2].iter().ge([1].iter()), true);
    assert_eq!([1, 2].iter().ge([1, 2].iter()), true);
}
