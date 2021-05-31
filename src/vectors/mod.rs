pub fn run() {
    println!("Learning vectors in Rust 🦀");

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    println!("{:?}", v);

    // ---

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for i in row {
        println!("{:?}", i);
    }

    let vec = vec![0; 5];
    assert_eq!(vec, [0, 0, 0, 0, 0]);

    // popped from the vec
    let mut vec = vec![1, 2, 3, 4, 5];
    while let Some(top) = vec.pop() {
        println!("{}", top);
    }

    /*
    * Implementations
    */

    println!("\n\rImplementation\n\r");

    /*
    * new
    *
    * Constructs a new, empty Vec<T>.
    */
    let mut vec: Vec<i32> = Vec::new();
    // vec[1] = 5; Panic! Out of bond
    // vec[6] = 9;
    vec.push(1);
    vec.push(2);
    println!("{:?}", vec);

    /*
    * with_capacity
    *
    * Constructs a new, empty Vec<T>.
    */
    let mut vec = Vec::with_capacity(10);

    // The vector contains no items, even though it has capacity for more
    assert_eq!(vec.len(), 0);
    assert_eq!(vec.capacity(), 10);

    // These are all done without reallocating...
    for i in 0..10 {
        vec.push(i);
    }
    println!("{:?}", vec);

    // ...but this may make the vector reallocate
    vec.push(11);
    println!("{}", vec.capacity());

    /*
    * capacity
    *
    * Returns the number of elements the vector can hold without reallocating.
    */
    // let vec: Vec<i32> = Vec::with_capacity(10);
    let vec: Vec<i32> = Vec::with_capacity(10);
    assert_eq!(vec.capacity(), 10);

    /*
    * reserve
    *
    * Reserves capacity for at least additional more elements 
    * to be inserted in the given Vec<T>.
    *
    * Panics if the new capacity exceeds isize::MAX bytes.
    */

    let mut vec = vec![1];
    vec.reserve(10);
    println!("{}",vec.capacity());

    /*
    * reserve_exact
    *
    * Reserves the minimum capacity for exactly additional more elements 
    * to be inserted in the given Vec<T>. 
    * After calling reserve_exact, capacity will be greater 
    * than or equal to self.len() + additional. 
    * Does nothing if the capacity is already sufficient.
    *
    * Panics if the new capacity overflows usize..
    */
    let mut vec = vec![1];
    vec.reserve_exact(10);
    println!("{}",vec.capacity());

    /*
    * shrink_to_fit
    *
    * Shrinks the capacity of the vector as much as possible.
    */
    let mut vec = Vec::with_capacity(10);
    vec.extend([1, 2, 3].iter().cloned());
    assert_eq!(vec.capacity(), 10);
    vec.shrink_to_fit();
    assert!(vec.capacity() >= 3);

    /*
    * into_boxed_slice
    *
    * Converts the vector into Box<[T]>.
    */
    let v = vec![1, 2, 3];
    let slice = v.into_boxed_slice();
    println!("{:?}", slice);

    /*
    * truncate
    *
    * Shortens the vector, keeping the first len elements and dropping the rest.
    * If len is greater than the vector’s current length, this has no effect.
    */
    let mut vec = vec![1, 2, 3, 4, 5];
    vec.truncate(2);
    assert_eq!(vec, [1, 2]);

    /*
    * as_slice
    *
    * Extracts a slice containing the entire vector.
    * Equivalent to &s[..].
    */
    let vec = vec![1, 2, 3, 5, 8];
    let buffer = vec.as_slice();
    let buffer2 = &vec[..];
    assert_eq!(buffer, buffer2);
    println!("{:?}", buffer);

    /*
    * as_mut_slice
    *
    * Extracts a mutable slice of the entire vector.
    * Equivalent to &mut s[..].
    */

    /*
    * as_ptr
    *
    * Returns a raw pointer to the vector’s buffer.
    *
    * How to use it? I can not imagine 🤷‍♂️
    */

    /*
    * swap_remove
    *
    * Removes an element from the vector and returns it.
    *
    * Panics if index is out of bounds.
    */
    let mut v = vec!["foo", "bar", "baz", "qux"];

    assert_eq!(v.swap_remove(1), "bar");
    assert_eq!(v, ["foo", "qux", "baz"]);

    assert_eq!(v.swap_remove(0), "foo");
    assert_eq!(v, ["baz", "qux"]);

    /*
    * isert
    *
    * Inserts an element at position index within the vector, 
    * shifting all elements after it to the right.
    *
    * Panics if index > len.
    */
    let mut vec = vec![1, 2, 3];
    vec.insert(1, 4);
    assert_eq!(vec, [1, 4, 2, 3]);
    vec.insert(4, 5);
    assert_eq!(vec, [1, 4, 2, 3, 5]);

    /*
    * remove
    *
    * Removes and returns the element at position index within the vector,
    * shifting all elements after it to the left.
    */
    let mut v = vec![1, 2, 3];
    assert_eq!(v.remove(1), 2);
    assert_eq!(v, [1, 3]);

    /*
    * retain
    *
    * Retains only the elements specified by the predicate.
    */
    let mut vec = vec![1, 2, 3, 4];
    vec.retain(|&x| x % 2 == 0);
    assert_eq!(vec, [2, 4]);

    /*
    * dedup_by_key
    *
    * Retains only the elements specified by the predicate.
    * If the vector is sorted, this removes all duplicates!!!!!!
    */
    println!();

    let mut vec = vec![10, 20, 21, 30, 20];
    vec.dedup_by_key(|i| *i / 10);
    println!("{:?}", vec);

    let mut vec = vec![10, 20, 20, 21, 30];
    vec.dedup_by_key(|i| *i / 10);
    println!("{:?}", vec);

    /*
    * push
    * 
    * Appends an element to the back of a collection.
    */
    let mut vec = vec![1, 2];
    vec.push(3);
    assert_eq!(vec, [1, 2, 3]);

    /*
    * pop
    * 
    * Removes the last element from a vector and returns it, or None if it is empty.
    */
    let mut vec = vec![1, 2, 3];
    assert_eq!(vec.pop(), Some(3));
    assert_eq!(vec, [1, 2]);

    /*
    * append
    * 
    * Moves all the elements of other into Self, leaving other empty.
    *
    * Panics if the number of elements in the vector overflows a usize.
    */
    let mut vec = vec![1, 2, 3];
    let mut vec2 = vec![4, 5, 6];
    vec.append(&mut vec2);
    assert_eq!(vec, [1, 2, 3, 4, 5, 6]);
    assert_eq!(vec2, []);

    /*
    * drain
    * 
    * Creates a draining iterator that removes the specified range in the vector
    * and yields the removed items.
    *
    * Panics if the starting point is greater than the end point
    * or if the end point is greater than the length of the vector.
    */
    let mut v = vec![1, 2, 3];
    let u: Vec<_> = v.drain(1..).collect();
    assert_eq!(v, &[1]);
    assert_eq!(u, &[2, 3]);

    // A full range clears the vector
    v.drain(..);
    assert_eq!(v, &[]);

    /*
    * clear
    * 
    * Clears the vector, removing all values.
    */
    let mut vec = vec![1, 2, 3];
    vec.clear();
    println!("{:?}", vec);

    /*
    * clear
    * 
    * Returns the number of elements in the vector,
    * also referred to as its ‘length’.
    */
    let a = vec![1, 2, 3];
    assert_eq!(a.len(), 3);

    /*
    * is_empty
    * 
    * Returns true if the vector contains no elements.
    */
    let mut v = Vec::new();
    assert!(v.is_empty());

    v.push(1);
    assert!(!v.is_empty());

    /*
    * split_off
    * 
    * Splits the collection into two at the given index.
    *
    * Panics if at > len.
    */
    let mut vec = vec![1, 2, 3];
    let vec2 = vec.split_off(1);
    assert_eq!(vec, [1]);
    assert_eq!(vec2, [2, 3]);

    /*
    * resize_with
    * 
    * Resizes the Vec in-place so that len is equal to new_len.
    */
    let mut vec = vec![1, 2, 3];
    vec.resize_with(5, Default::default);
    assert_eq!(vec, [1, 2, 3, 0, 0]);

    let mut vec = Vec::with_capacity(4);
    let mut p = 1;
    vec.resize_with(4, || { p *= 2; p });
    // assert_eq!(vec, [2, 4, 8, 16]);
    println!("{:?}", vec);
    println!("{}", vec.capacity());

    /*
    * leak
    * 
    * Consumes and leaks the Vec, returning a mutable 
    * reference to the contents, &'a mut [T]
    */
    let x = vec![1, 2, 3];
    let static_ref: &'static mut [usize] = x.leak();
    static_ref[0] += 1;
    assert_eq!(static_ref, &[2, 2, 3]);

    /*
    * resize
    * 
    * Resizes the Vec in-place so that len is equal to new_len.
    */
    let mut vec = vec!["hello"];
    vec.resize(3, "world");
    assert_eq!(vec, ["hello", "world", "world"]);

    let mut vec = vec![1, 2, 3, 4];
    vec.resize(2, 0);
    assert_eq!(vec, [1, 2]);

    /*
    * extend_from_slice
    * 
    * Clones and appends all elements in a slice to the Vec.
    */
    let mut vec = vec![1];
    vec.extend_from_slice(&[2, 3, 4]);
    assert_eq!(vec, [1, 2, 3, 4]);

    /*
    * dedup
    * 
    * Removes consecutive repeated elements in the vector 
    * according to the PartialEq trait implementation.
    * If the vector is sorted, this removes all duplicates.
    */
    let mut vec = vec![1, 2, 2, 3, 2];
    vec.dedup();
    println!("{:?}", vec);

    let mut vec = vec![1, 2, 2, 3, 3, 5, 5, 7];
    vec.dedup();
    println!("{:?}", vec);

    /*
    * splice
    * 
    * Creates a splicing iterator that replaces the specified range 
    * in the vector with the given replace_with iterator and yields 
    * the removed items. 
    * replace_with does not need to be the same length as range.
    *
    * Panics if the starting point is greater than the end point 
    * or if the end point is greater than the length of the vector.
    */
    let mut v = vec![1, 2, 3];
    let new = [7, 8];
    let u: Vec<_> = v.splice(..2, new.iter().cloned()).collect();
    assert_eq!(v, &[7, 8, 3]);
    assert_eq!(u, &[1, 2]);

    /*
    * len
    * 
    * Returns the number of elements in the slice.
    */
    let a = [1, 2, 3];
    assert_eq!(a.len(), 3);

    /*
    * is_empty
    * 
    * Returns true if the slice has a length of 0.
    */
    let vec: Vec<i32> = Vec::new();
    if vec.is_empty() {
        println!("vector is empty");
    }

    /*
    * first
    * 
    * Returns the first element of the slice, or None if it is empty.
    */
    let v = vec![10, 40, 30];
    assert_eq!(Some(&10), v.first());

    let w: Vec<i32> = Vec::new();
    assert_eq!(None, w.first());

    /*
    * first_mut
    * 
    * Returns a mutable pointer to the first element of the slice,
    * or None if it is empty.
    */
    let x = &mut vec![0, 1, 2];

    if let Some(first) = x.first_mut() {
        *first = 5;
    }
    assert_eq!(x, &[5, 1, 2]);

    /*
    * split_first
    * 
    * Returns the first and all the rest of the elements of the slice,
    * or None if it is empty.
    */
    let x = vec![0, 1, 2, 3, 5];

    if let Some((first, elements)) = x.split_first() {
        println!("{:?}", first);
        println!("{:?}", elements);
    }

    /*
    * split_first_mut
    * 
    * Returns the first and all the rest of the elements of the slice,
    * or None if it is empty.
    */
    println!();

    let mut x = vec![0, 1, 2, 3, 5];

    if let Some((first, elements)) = x.split_first_mut() {
        *first = 3;
        elements[0] = 4;
        elements[1] = 5;
    }
    println!("{:?}", x);

    /*
    * split_last
    * 
    * Returns the last and all the rest of the elements of the slice,
    * or None if it is empty.
    */
    println!();

    let x = vec![0, 1, 2];

    if let Some((last, elements)) = x.split_last() {
        println!("{:?}", last);
        println!("{:?}", elements);
    }

    /*
    * split_last_mut
    * 
    * Returns the last and all the rest of the elements of the slice,
    * or None if it is empty.
    */
    let x = &mut [0, 1, 2];

    if let Some((last, elements)) = x.split_last_mut() {
        *last = 3;
        elements[0] = 4;
        elements[1] = 5;
    }
    assert_eq!(x, &[4, 5, 3]);

    /*
    * last
    * 
    * Returns the last element of the slice, or None if it is empty.
    */
    let vec = vec![1, 2, 3];
    let last = vec.last();
    println!("The last element of vector is: {:?}", last);

    /*
    * last_mut
    * 
    * Returns a mutable pointer to the last item in the slice.
    */
    let x = &mut [0, 1, 2];

    if let Some(last) = x.last_mut() {
        *last = 10;
    }
    assert_eq!(x, &[0, 1, 10]);

    /*
    * get
    * 
    * Returns a reference to an element or subslice depending on the type of index.
    */
    let v = [10, 40, 30];
    assert_eq!(Some(&40), v.get(1));
    assert_eq!(Some(&[10, 40][..]), v.get(0..2));
    assert_eq!(None, v.get(3));
    assert_eq!(None, v.get(0..4));

    /*
    * get_mut
    * 
    * Returns a mutable reference to an element
    * or subslice depending on the type of index (see get) 
    * or None if the index is out of bounds.
    */
    let x = &mut [0, 1, 2];

    if let Some(elem) = x.get_mut(1) {
        *elem = 42;
    }
    assert_eq!(x, &[0, 42, 2]);

    /*
    * as_ptr
    * 
    * Returns a raw pointer to the slice’s buffer.
    */
    let x = &[1, 2, 4];
    let x_ptr = x.as_ptr();

    unsafe {
        for i in 0..x.len() {
            assert_eq!(x.get_unchecked(i), &*x_ptr.add(i));
        }
    }

    /*
    * swap
    * 
    * Swaps two elements in the slice.
    */
    let mut v = vec!["a", "b", "c", "d"];
    v.swap(1, 3);
    println!("{:?}", v);

    /*
    * reverse
    * 
    * Reverses the order of elements in the slice, in place.
    */
    let mut v = vec![1, 2, 3];
    v.reverse();
    println!("{:?}", v);

    /*
    * iter
    * 
    * Returns an iterator over the slice.
    */
    let x = &[1, 2, 4];
    let mut iterator = x.iter();

    assert_eq!(iterator.next(), Some(&1));
    assert_eq!(iterator.next(), Some(&2));
    assert_eq!(iterator.next(), Some(&4));
    assert_eq!(iterator.next(), None);

    /*
    * iter_mut
    * 
    * Returns an iterator that allows modifying each value.
    */
    let x = &mut [1, 2, 4];
    for elem in x.iter_mut() {
        *elem += 2;
    }
    assert_eq!(x, &[3, 4, 6]);

    /*
    * windows
    * 
    * Returns an iterator over all contiguous windows of length size. 
    * The windows overlap. If the slice is shorter than size, 
    * the iterator returns no values.
    */
    let slice = ['r', 'u', 's', 't'];
    let mut iter = slice.windows(2);
    assert_eq!(iter.next().unwrap(), &['r', 'u']);
    assert_eq!(iter.next().unwrap(), &['u', 's']);
    assert_eq!(iter.next().unwrap(), &['s', 't']);
    assert!(iter.next().is_none());

    /*
    * chunks
    * 
    * Returns an iterator over chunk_size elements of the slice at a time,
    * starting at the beginning of the slice.
    *
    * Panics if chunk_size is 0.
    */
    println!();
    let slice = vec!['l', 'o', 'r', 'e', 'm'];
    for item in slice.chunks(2) {
        println!("{:?}", item);
    }

    /*
    * chunks_mut
    * 
    * Returns an iterator over chunk_size elements of the slice at a time,
    * starting at the beginning of the slice.
    *
    * Panics if chunk_size is 0.
    */
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;

    for chunk in v.chunks_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[1, 1, 2, 2, 3]);

    /*
    * rchunks
    * 
    * Returns an iterator over chunk_size elements of the slice at a time,
    * starting at the end of the slice.
    *
    * Panics if chunk_size is 0.
    */
    println!();
    let slice = vec!['l', 'o', 'r', 'e', 'm'];
    for item in slice.rchunks(2) {
        println!("{:?}", item);
    }

    /*
    * rchunks_mut
    * 
    * Returns an iterator over chunk_size elements of the slice at a time,
    * starting at the end of the slice.
    *
    * Panics if chunk_size is 0.
    */
    let v = &mut [0, 0, 0, 0, 0];
    let mut count = 1;

    for chunk in v.rchunks_mut(2) {
        for elem in chunk.iter_mut() {
            *elem += count;
        }
        count += 1;
    }
    assert_eq!(v, &[3, 2, 2, 1, 1]);

    /*
    * split_at
    * 
    * Divides one slice into two at an index.
    *
    * Panics if mid > len.
    */
    let v = vec![1, 2, 3, 4, 5, 6];

    let (left, right) = v.split_at(0);
    assert_eq!(left, []);
    assert_eq!(right, [1, 2, 3, 4, 5, 6]);

    let (left, right) = v.split_at(2);
    assert_eq!(left, [1, 2]);
    assert_eq!(right, [3, 4, 5, 6]);
 
    let (left, right) = v.split_at(6);
    assert_eq!(left, [1, 2, 3, 4, 5, 6]);
    assert_eq!(right, []);

    /*
    * split_at_mut
    * 
    * Divides one mutable slice into two at an index
    *
    * Panics if mid > len.
    */
    let mut v = [1, 0, 3, 0, 5, 6];
    let (left, right) = v.split_at_mut(2);
    assert_eq!(left, [1, 0]);
    assert_eq!(right, [3, 0, 5, 6]);
    left[1] = 2;
    right[1] = 4;
    assert_eq!(v, [1, 2, 3, 4, 5, 6]);

    /*
    * split
    * 
    * Returns an iterator over subslices separated by elements that match pred.
    * The matched element is not contained in the subslices.
    */
    let slice = [10, 40, 33, 20];
    let mut iter = slice.split(|num| num % 3 == 0);

    assert_eq!(iter.next().unwrap(), &[10, 40]);
    assert_eq!(iter.next().unwrap(), &[20]);
    assert!(iter.next().is_none());

    /*
    * split_mut
    * 
    * Returns an iterator over mutable subslices separated by elements 
    * that match pred. The matched element is not contained in the subslices.
    */
    println!();
    let mut v = [10, 40, 30, 20, 60, 50];

    for group in v.split_mut(|num| *num % 3 == 0) {
        group[0] = 1;
    }
    assert_eq!(v, [1, 40, 30, 1, 60, 1]);

    /*
    * split_inclusive
    * 
    * Returns an iterator over subslices separated by elements that match pred.
    * The matched element is contained in the end of the previous subslice 
    * as a terminator.
    */
    let slice = [10, 40, 33, 20];
    let mut iter = slice.split_inclusive(|num| num % 3 == 0);

    assert_eq!(iter.next().unwrap(), &[10, 40, 33]);
    assert_eq!(iter.next().unwrap(), &[20]);
    assert!(iter.next().is_none());

    /*
    * split_inclusive_mut
    * 
    * Returns an iterator over mutable subslices separated by elements 
    * that match pred. The matched element is contained in the previous 
    * subslice as a terminator.
    */
    let mut v = [10, 40, 30, 20, 60, 50];

    for group in v.split_inclusive_mut(|num| *num % 3 == 0) {
        let terminator_idx = group.len()-1;
        group[terminator_idx] = 1;
    }
    assert_eq!(v, [10, 40, 1, 20, 1, 1]);

    /*
    * rsplit
    * 
    * Returns an iterator over subslices separated by elements that match pred, 
    * starting at the end of the slice and working backwards. 
    * The matched element is not contained in the subslices.
    */
    let slice = [11, 22, 33, 0, 44, 55];
    let mut iter = slice.rsplit(|num| *num == 0);

    assert_eq!(iter.next().unwrap(), &[44, 55]);
    assert_eq!(iter.next().unwrap(), &[11, 22, 33]);
    assert_eq!(iter.next(), None);

    /*
    * rsplit_mut
    * 
    * Returns an iterator over mutable subslices separated by elements that match pred,
    * starting at the end of the slice and working backwards. 
    * The matched element is not contained in the subslices.
    */
    let mut v = [100, 400, 300, 200, 600, 500];

    let mut count = 0;
    for group in v.rsplit_mut(|num| *num % 3 == 0) {
        count += 1;
        group[0] = count;
    }
    assert_eq!(v, [3, 400, 300, 2, 600, 1]);

    /*
    * splitn
    * 
    * Returns an iterator over subslices separated by elements that match pred,
    * limited to returning at most n items. The matched element is not contained in the subslices.
    * The last element returned, if any, will contain the remainder of the slice.
    */
    let v = [10, 40, 30, 20, 60, 50];

    for group in v.splitn(2, |num| *num % 3 == 0) {
        println!("{:?}", group);
    }

    /*
    * splitn_mut
    * 
    * Returns an iterator over subslices separated by elements that match pred, 
    * limited to returning at most n items. 
    * The matched element is not contained in the subslices.
    *
    * The last element returned, if any, will contain the remainder of the slice.
    */
    let mut v = [10, 40, 30, 20, 60, 50];

    for group in v.splitn_mut(2, |num| *num % 3 == 0) {
        group[0] = 1;
    }
    assert_eq!(v, [1, 40, 30, 1, 60, 50]);

    /*
    * rsplitn
    * 
    * Returns an iterator over subslices separated by elements that match pred 
    * limited to returning at most n items. 
    * This starts at the end of the slice and works backwards. 
    * The matched element is not contained in the subslices.
    * The last element returned, if any, will contain the remainder of the slice.
    */
    let v = [10, 40, 30, 20, 60, 50];

    for group in v.rsplitn(2, |num| *num % 3 == 0) {
        println!("{:?}", group);
    }

    /*
    * rsplitn_mut
    * 
    * Returns an iterator over subslices separated by elements that match 
    * pred limited to returning at most n items. 
    * This starts at the end of the slice and works backwards. 
    * The matched element is not contained in the subslices.
    * The last element returned, if any, will contain the remainder of the slice.
    */
    let mut s = [10, 40, 30, 20, 60, 50];

    for group in s.rsplitn_mut(2, |num| *num % 3 == 0) {
        group[0] = 1;
    }
    assert_eq!(s, [1, 40, 30, 20, 60, 1]);

    /*
    * contains
    * 
    * Returns true if the slice contains an element with the given value.
    */
    let v = vec![10, 40, 30];
    assert!(v.contains(&30));
    assert!(!v.contains(&50));

    /*
    * starts_with
    * 
    * Returns true if needle is a prefix of the slice.
    */
    let v = [10, 40, 30];
    assert!(v.starts_with(&[10]));
    assert!(v.starts_with(&[10, 40]));
    assert!(!v.starts_with(&[50]));
    assert!(!v.starts_with(&[10, 50]));

    // Always returns true if needle is an empty slice:
    let v = &[10, 40, 30];
    assert!(v.starts_with(&[]));
    let v: &[u8] = &[];
    assert!(v.starts_with(&[]));

    /*
    * ends_with
    * 
    * Returns true if needle is a suffix of the slice.
    */
    let v = [10, 40, 30];
    assert!(v.ends_with(&[30]));
    assert!(v.ends_with(&[40, 30]));
    assert!(!v.ends_with(&[50]));
    assert!(!v.ends_with(&[50, 30]));
    
    // Always returns true if needle is an empty slice:
    let v = &[10, 40, 30];
    assert!(v.ends_with(&[]));
    let v: &[u8] = &[];
    assert!(v.ends_with(&[]));

    /*
    * strip_prefix
    * 
    * Returns a subslice with the prefix removed.
    */
    let v = &[10, 40, 30];
    assert_eq!(v.strip_prefix(&[10]), Some(&[40, 30][..]));
    assert_eq!(v.strip_prefix(&[10, 40]), Some(&[30][..]));
    assert_eq!(v.strip_prefix(&[50]), None);
    assert_eq!(v.strip_prefix(&[10, 50]), None);

    let prefix : &str = "he";
    assert_eq!(b"hello".strip_prefix(prefix.as_bytes()),
        Some(b"llo".as_ref()));

    /*
    * strip_prefix
    * 
    * Returns a subslice with the prefix removed.
    */
    let v = &[10, 40, 30];
    assert_eq!(v.strip_suffix(&[30]), Some(&[10, 40][..]));
    assert_eq!(v.strip_suffix(&[40, 30]), Some(&[10][..]));
    assert_eq!(v.strip_suffix(&[50]), None);
    assert_eq!(v.strip_suffix(&[50, 30]), None);

    /*
    * binary_search
    * 
    * Binary searches this sorted slice for a given element.
    */
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

    assert_eq!(s.binary_search(&13),  Ok(9));
    assert_eq!(s.binary_search(&4),   Err(7));
    assert_eq!(s.binary_search(&100), Err(13));
    let r = s.binary_search(&1);
    assert!(match r { Ok(1..=4) => true, _ => false, });

    // If you want to insert an item to a sorted vector, while maintaining sort order:

    let mut s = vec![0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
    let num = 42;
    let idx = s.binary_search(&num).unwrap_or_else(|x| x);
    s.insert(idx, num);
    assert_eq!(s, [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 42, 55]);

    /*
    * binary_search_by
    * 
    * Binary searches this sorted slice with a comparator function.
    */
    let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];

    let seek = 13;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Ok(9));
    let seek = 4;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(7));
    let seek = 100;
    assert_eq!(s.binary_search_by(|probe| probe.cmp(&seek)), Err(13));
    let seek = 1;
    let r = s.binary_search_by(|probe| probe.cmp(&seek));
    assert!(match r { Ok(1..=4) => true, _ => false, });

    /*
    * binary_search_by_key
    * 
    * Binary searches this sorted slice with a key extraction function.
    */
    let s = [(0, 0), (2, 1), (4, 1), (5, 1), (3, 1),
         (1, 2), (2, 3), (4, 5), (5, 8), (3, 13),
         (1, 21), (2, 34), (4, 55)];

    assert_eq!(s.binary_search_by_key(&13, |&(a, b)| b),  Ok(9));
    assert_eq!(s.binary_search_by_key(&4, |&(a, b)| b),   Err(7));
    assert_eq!(s.binary_search_by_key(&100, |&(a, b)| b), Err(13));
    let r = s.binary_search_by_key(&1, |&(a, b)| b);
    assert!(match r { Ok(1..=4) => true, _ => false, });

    /*
    * sort_unstable
    * 
    * Sorts the slice, but may not preserve the order of equal elements.
    */
    let mut v = [-5, 4, 1, -3, 2];

    v.sort_unstable();
    assert!(v == [-5, -3, 1, 2, 4]);

    /*
    * sort_unstable_by
    * 
    * Sorts the slice with a comparator function, 
    * but may not preserve the order of equal elements.
    */
    let mut floats = [5f64, 4.0, 1.0, 3.0, 2.0];
    floats.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    assert_eq!(floats, [1.0, 2.0, 3.0, 4.0, 5.0]);

    /*
    * rotate_left
    * 
    * Rotates the slice in-place such that the first mid elements of the slice 
    * move to the end while the last
    */
    let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
    a.rotate_left(2);
    assert_eq!(a, ['c', 'd', 'e', 'f', 'a', 'b']);

    // Rotating a subslice:
    let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
    a[1..5].rotate_left(1);
    assert_eq!(a, ['a', 'c', 'd', 'e', 'b', 'f']);

    /*
    * rotate_right
    */
    let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
    a.rotate_right(2);
    assert_eq!(a, ['e', 'f', 'a', 'b', 'c', 'd']);

    // Rotate a subslice:
    let mut a = ['a', 'b', 'c', 'd', 'e', 'f'];
    a[1..5].rotate_right(1);
    assert_eq!(a, ['a', 'e', 'b', 'c', 'd', 'f']);

    /*
    * fill
    *
    * Fills self with elements by cloning value.
    */
    let mut buf = vec![0; 10];
    buf.fill(1);
    println!("{:?}", buf);

    /*
    * fill_with
    *
    * Fills self with elements returned by calling a closure repeatedly.
    */
    let mut buf = vec![0; 10];
    let i = 1;
    buf.fill_with(|| i + 5);
    println!("{:?}", buf);
    buf.fill_with(Default::default);
    println!("{:?}", buf);

    /*
    * clone_from_slice
    *
    * Copies the elements from src into self.
    *
    * This function will panic if the two slices have different lengths.
    */
    let src = [1, 2, 3, 4];
    let mut dst = [0, 0];

    // Because the slices have to be the same length,
    // we slice the source slice from four elements
    // to two. It will panic if we don't do this.
    dst.clone_from_slice(&src[2..]);

    assert_eq!(src, [1, 2, 3, 4]);
    assert_eq!(dst, [3, 4]);

    /*
    * copy_from_slice
    *
    * Copies all elements from src into self, using a memcpy.
    *
    * This function will panic if the two slices have different lengths.
    */
    let src = [1, 2, 3, 4];
    let mut dst = [0, 0];

    // Because the slices have to be the same length,
    // we slice the source slice from four elements
    // to two. It will panic if we don't do this.
    dst.copy_from_slice(&src[2..]);

    assert_eq!(src, [1, 2, 3, 4]);
    assert_eq!(dst, [3, 4]);

    /*
    * copy_within
    *
    * Copies elements from one part of the slice to another part of itself, 
    * using a memmove.
    */
    let mut bytes = *b"Hello, World!";
    bytes.copy_within(1..5, 8);
    assert_eq!(&bytes, b"Hello, Wello!");

    /*
    * swap_with_slice
    *
    * This function will panic if the two slices have different lengths.
    */
    let mut slice1 = [0, 0];
    let mut slice2 = [1, 2, 3, 4];

    slice1.swap_with_slice(&mut slice2[2..]);

    assert_eq!(slice1, [3, 4]);
    assert_eq!(slice2, [1, 2, 0, 0]);

    /*
    * is_ascii
    *
    * Checks if all bytes in this slice are within the ASCII range.
    */
    // let vec = vec!['a', 'b', 'c'];
    // assert_eq!(true, vec.is_ascii());

    /*
    * sort
    *
    * Sorts the slice.
    */
    let mut v = vec![-5, 4, 1, -3, 2];
    v.sort();
    println!("{:?}", v);

    /*
    * sort_by
    *
    * Sorts the slice with a comparator function.
    */
    let mut v = [5, 4, 1, 3, 2];
    v.sort_by(|a, b| a.cmp(b));
    assert!(v == [1, 2, 3, 4, 5]);

    // reverse sorting
    v.sort_by(|a, b| b.cmp(a));
    assert!(v == [5, 4, 3, 2, 1]);

    /*
    * sort_by_key
    *
    * Sorts the slice with a key extraction function.
    */
    let mut v = [-5i32, 4, 1, -3, 2];

    v.sort_by_key(|k| k.abs());
    assert!(v == [1, 2, -3, 4, -5]);

    /*
    * to_vec
    *
    * Copies self into a new Vec.
    */
    let s = [10, 40, 30];
    let x = s.to_vec();
    println!("{:?}", x);

    /*
    * concat
    *
    * Copies self into a new Vec.
    */
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let vec3 = [vec1, vec2].concat();
    println!("{:?}", vec3);

    /*
    * join
    *
    * Flattens a slice of T into a single value Self::Output,
    * placing a given separator between each.
    */
    println!("{:?}", ["hello", "world"].join(" "));
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}