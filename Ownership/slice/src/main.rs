fn main() {
    /*
    Difference between String and &str
    String is a growable, mutable, owned, UTF-8 encoded string type.
    &str is a slice, a reference to some UTF-8 encoded string data stored elsewhere.
    */
    let s: String = String::from("hello world");
    let s1: &str = "hello, &str world.";
    let word: usize = first_word(&s); // &s is also a &str
    let better_word: &str = better_first_word(&s); // &s is also a &str
    println!("The end index of first word is: {}", word);
    println!("The first word is: {}", better_word);
    println!("The first word is: {}", better_first_word(s1));

    // String slice
    let hello: &str = &s[0..5]; // [0..5] is equal to [0..=4] & [..5]
    let world: &str = &s[6..11]; // [6..11] is equal to [6..=10] & [6..]
    let hello_world: &str = &s[..]; // [..] is equal to [0..=10] & [0..]
    println!("{} {}, {}", hello, world, hello_world);

    // Array slice
    let a: [i32; 5] = [4, 2, 0, 6, 9];
    let slice: &[i32] = &a[1..3]; // [1..3] is equal to [1..=2] & [1..]
    println!("The slice is: {:?}", slice);
}

// This function returns the first word of a string slice.
fn first_word(s: &String) -> usize {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// Better version of first_word, using string slice
fn better_first_word(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
