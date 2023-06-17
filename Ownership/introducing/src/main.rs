/*
    Ownership Rules:
    1. Each value in Rust has an owner.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
*/

fn main() {
    {
        // s is not valid here, it's not yet declared
        let s: String = String::from("hello world yeah yeah"); // s is a valid from this point forward
        takes_ownership(s); /* do stuff with s */
        // println!("{}", s); // this will not work

        let x: i32 = 5; // x is a valid from this point forward
        makes_copy(x); // do stuff with x
        println!("the value of x: {}", x); // this will work

        let s1: String = gives_ownership();
        println!("s1: {}", s1);
    } // this scope is now over, and s is no longer valid

    {
        let x: i32 = 5;
        let y: i32 = x; // x is copied to y

        let s1: String = String::from("hello");
        let s2: String = s1; // s1 is moved to s2
        let s3: String = s2.clone(); // s2 is cloned to s3

        // println!("x: {}, y: {}. {}, world", x, y, s1); // this will not work
        println!("x: {}, y: {}. {}, world", x, y, s3); // this will work
    }
    {
        let s1: String = gives_ownership();
        let s2: String = String::from("s2");
        let s3: String = takes_and_gives_back(s2);
        println!("s1: {}, s3: {}", s1, s3); // can't use s2 here, cause it's moved to s3
    }
}

fn takes_ownership(some_string: String) {
    println!("taking ownership of: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("making copy of: {}", some_integer);
}

fn gives_ownership() -> String {
    // This function will move its return value into the function that calls it
    let some_string: String = String::from("hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
