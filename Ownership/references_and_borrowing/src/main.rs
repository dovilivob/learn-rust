fn main() {
    /* Overall rules:
    1. At any given time, you can have either one mutable reference or multiple immutable references
    2. References must always be valid   */


    // declare a immutable String var
    let s1: String = String::from("hello, s1");
    // call a function and pass s1 as a reference
    let len = calculate_length(&s1);
    // s1 is still valid here
    println!("The length of '{}' is {}.", s1, len);

    // declare a mutable String var
    let mut s2: String = String::from("hello, s2");
    // call a function and pass s2 as a mutable reference
    change(&mut s2);
    // s2 is still valid here
    println!("{}", s2);

    /*
    let r1: &mut String = &mut s2;
    let r2: &mut String = &mut s2; // Error: cannot borrow `s2` as mutable more than once at a time
    */

    /* Data Race: When 2 pointers points to the same piece of data */
    // Data Race is bad because it can cause a lot of shitty things
    // But we can make it safe by using the following rules:
    // 1. You can have either one mutable reference or multiple immutable references
    // 2. References must always be valid

    let r1: &String = &s2;
    let r2: &String = &s2;
    // It's okay to have multiple immutable references, because it won't change when other references are using it

    // this can't happen because r1 and r2 are still alive, and the rule says: you can have either one mutable reference or multiple immutable references
    // let r3: &mut String = &mut s2;

    println!("{}, {}", r1, r2); // if r1, r2 never use in the rest of codes, the scope of them ends here.

    // It's ok to declare a mutable reference here, because r1 and r2 are not alive anymore
    let r3: &mut String = &mut s2;
    println!("{}", r3); // The scope of r1 and r2 ends here


    // let reference_to_nothing: &String = dangle(); // Error: borrowed value does not live long enough
}

// Pass by reference, which is borrowing
fn calculate_length(s: &String) -> usize {
    // s.push_str(" world"); // Error: cannot borrow `*s` as mutable, as it is behind a `&` reference
    let length: usize = s.len();
    length
}

// Make a mutable reference, which can be changed
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn dangle() -> &String {
    let s: String = String::from("hello");
    &s // Error: `s` does not live long enough
} // `s` dropped here while still borrowed
*/