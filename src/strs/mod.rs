use std::str;

pub fn run() {
    print!("{}", "Learning strs...\n\n");

    /*
     * Implementations
     */

    /*
     * as_bytes
     * Converts a string slice to a byte slice.
     */

    let s = "hello";
    let bytes = s.as_bytes();
    println!("{:?}", bytes);

    let s = str::from_utf8(bytes).unwrap();
    println!("{}", s);

    println!();

    /*
     * as_ptr
     *
     * Converts a string slice to a raw pointer.
     */
    let s = "Hello";
    let ptr = s.as_ptr();
    println!("{:?}", ptr);

    println!();

    /*
     * bytes
     *
     * An iterator over the bytes of a string slice.
     */
    let s = "rose";
    for byte in s.bytes() {
        print!("{} ", byte);
    }
    println!();
    println!("{} {} {} {}", b'r', b'o', b's', b'e');

    println!();

    /*
     * char_indices
     *
     * Returns an iterator over the chars of a string slice, and their positions.
     */
    let s = "Привет! ❤️";
    for (i, item) in s.char_indices() {
        println!("{} - {}", i, item);
    }

    println!();

    /*
     * chars
     *
     * Returns an iterator over the chars of a string slice.
     */
    let s = "mountain";
    for item in s.chars() {
        println!("{}", item);
    }

    println!("The word `{}` contains {} chars", s, s.chars().count());

    let s = "привет";
    for item in s.chars() {
        println!("{}", item);
    }

    println!();

    /*
     * contains
     *
     * Returns true if the given pattern matches a sub-slice of this string slice.
     */

    let s = "my destiny";
    if s.contains("my") {
        println!("`my destiny` contains `my`");
    }

    println!();

    /*
     * end_with
     *
     * Returns true if the given pattern matches a suffix of this string slice.
     */

    let s = "ananas";
    if s.ends_with("nas") {
        println!("Yep, `ananas` ends with `nas`");
    }

    println!();

    /*
     * eq_ignore_ascii_case
     *
     * Checks that two strings are an ASCII case-insensitive match.
     */
    assert!("Ferris".eq_ignore_ascii_case("fErRiS"));
    assert!("Ferrös".eq_ignore_ascii_case("FERRöS"));
    assert!(!"Ferrös".eq_ignore_ascii_case("FERRÖS"));

    /*
     * escape_debug
     *
     * Return an iterator that escapes each char in self with char::escape_debug.
     */
    for c in "❤\n!".escape_debug() {
        print!("{}", c);
    }
    println!();

    /*
     * escape_debug
     *
     * Return an iterator that escapes each char in self with char::escape_default.
     */
    println!("{}", "❤\n!".escape_default());
    println!();

    /*
     * escape_unicode
     *
     * Return an iterator that escapes each char in self with char::escape_unicode.
     */
    println!("{}", "❤\n!".escape_unicode());
    println!();

    /*
     * find
     *
     * Returns the byte index of the first character of this string
     * slice that matches the pattern.
     */
    let s = "Löwe 老虎 Léopard Gepardi";
    println!("{}", s);
    println!("{:?}", s.find("Léo"));
    println!("{:?}", s.find(char::is_whitespace));
    // with closure
    println!("{:?}", s.find(|c: char| c.is_lowercase() || c == 'w'));

    println!();

    /*
     * get
     *
     * Returns a subslice of str.
     * Returns None whenever equivalent indexing operation would panic.
     */

    let s = "🔥 123 abc";
    println!("{:?}", s.get(0..4));
    println!("{:?}", s.get(0..3));

    println!();

    /*
     * get_mut
     *
     * Returns a mutable subslice of str.
     */
    let mut s = String::from("hello");
    let sub = s.get_mut(0..5);
    let sub = sub.map(|c| {
        c.make_ascii_uppercase();
        &*c
    });
    println!("{:?}", sub);

    println!();

    /*
     * is_ascii
     *
     * Checks if all characters in this string are within the ASCII range.
     */
    let non_ascii = "Grüße, Jürgen ❤";
    let ascii = "hello!";

    if !non_ascii.is_ascii() {
        println!("`{}` is not ASCII 🙁", non_ascii);
    }
    if ascii.is_ascii() {
        println!("`{}` is ASCII 🙂", ascii);
    }

    println!();

    /*
     * is_empty
     *
     * Returns true if self has a length of zero bytes.
     */
    let s = "";
    if s.is_empty() {
        println!("The string is empty 🤷‍♂️");
    }

    println!();

    /*
     * len
     *
     * Returns the length of self.
     */
    let len = "foo".len();
    println!("{}", len);

    println!();

    /*
     * lines
     *
     * An iterator over the lines of a string, as string slices.
     */
    let s = "a\nb\nc\n";
    let lines = s.lines().count();
    println!("`{}` has {} lines", s.escape_debug(), lines);

    println!();

    /*
     * make_ascii_lowercase
     *
     * Converts this string to its ASCII lower case equivalent in-place.
     */
    let mut s = String::from("aBcMniOOp");
    s.make_ascii_lowercase();
    println!("{:?}", s);

    println!();

    /*
     * make_ascii_uppercase
     *
     * Converts this string to its ASCII lower case equivalent in-place.
     */
    let mut s = String::from("aBcMniOOp");
    s.make_ascii_uppercase();
    println!("{:?}", s);

    println!();

    /*
     * match_indices
     *
     * An iterator over the disjoint matches of a pattern within this string slice
     * as well as the index that the match starts at.
     */
    let s = "UIu23yabc u23y 989 abc IUy23y 23 2nnnnabc99";
    let matches: Vec<_> = s.match_indices("abc").collect();

    for (i, _item) in matches {
        println!("{}", i);
    }

    println!();

    /*
     * matches
     *
     * An iterator over the disjoint matches of a pattern within the given string slice.
     */
    let s = "UIu23yabc u23y 989 abc IUy23y 23 2nnnnabc99";
    let matches: Vec<_> = s.matches(char::is_numeric).collect();

    for item in matches {
        println!("{}", item);
    }

    println!();

    /*
     * parse
     *
     * Parses this string slice into another type.
     */
    let num: u32 = "4".parse().unwrap();
    assert_eq!(4, num);

    // Using the ‘turbofish’ instead of annotating four:
    let num = "4".parse::<u32>();
    println!("{:?}", num);

    println!();

    /*
     * repeat
     *
     * Creates a new String by repeating a string n times.
     */
    println!("{}", "abc".repeat(4));

    println!();

    /*
     * replace
     *
     * Replaces all matches of a pattern with another string.
     */
    let s1 = "this is old";
    let s2 = s1.replace("old", "new");
    println!("{}", s1);
    println!("{}", s2);

    println!();

    /*
     * replacen
     *
     * Replaces first N matches of a pattern with another string.
     */
    let s1 = "this is old, old, very old";
    let s2 = s1.replacen("old", "new", 2);
    println!("{}", s1);
    println!("{}", s2);

    println!();
}
