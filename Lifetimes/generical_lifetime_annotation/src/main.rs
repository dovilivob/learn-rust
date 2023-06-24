fn main() {
    let string1 = String::from("abcd");
    let result: &str;
    {
        let string2 = String::from("xyz");

        // we can change result value here even if it's immutable, because we're changing the reference
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
        let result = longest_no_lifetime_annotation(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    // println!("The longest string is {}", result); // this won't work because string2 is out of scope
}

// &i32       // a reference
// &'a i32    // a reference with an explicit lifetime
// &'a mut i32// a mutable reference with an explicit lifetime

// Lifetime annotations don’t change how long any of the references live.
// Just as functions can accept any type when the signature specifies a generic type parameter,
// functions can accept references with any lifetime by specifying a generic lifetime parameter.
// Lifetime annotations describe the relationships of the lifetimes of multiple references to each other
// without affecting the lifetimes.

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// we need to care about lifetime when we have a function that returns a reference,
// because the lifetime of the return value must be the same as the smaller of the lifetimes of the two parameters.
// If the reference returned does not refer to one of the parameters, it must refer to a value created within this function,
// which would be a dangling reference because the value will go out of scope at the end of the function.
// so if the return type is a String, which isn't a reference, we don't need to care about lifetime.
fn longest_no_lifetime_annotation(x: &str, y: &str) -> String {
    if x.len() > y.len() {
        x.to_string()
    } else {
        y.to_string()
    }
}

/*
// this won't work because the compiler doesn't know which lifetime to use
fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        return x;
    } else {
        y
    }
}
*/
