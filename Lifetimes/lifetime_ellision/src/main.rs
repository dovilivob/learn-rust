// Lifetime ellision rules
// 1. Each parameter that is a reference gets its own lifetime parameter.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
// 3. If there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method,
//    the lifetime of self is assigned to all output lifetime parameters.

fn main() {
    println!("Hello, world!");
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        // enumerate() returns a tuple (index, reference)
        if item == b' ' {
            // b' ' is a byte literal
            return &s[0..i]; // return a slice of s from index 0 to i
        }
    }
    &s[..] // return a slice of s from index 0 to the end
}
