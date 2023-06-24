use unicode_segmentation::UnicodeSegmentation;

fn main() {
    // Strings are stored as a collection of UTF-8 encoded bytes in rust
    {
        let _s1: String = String::new();
        let s2: &str = "yeah";
        let _s3: String = s2.to_string();
        let _s4: String = String::from("yeah yeah");
    }
    {
        let mut s = String::from("yeah");
        s.push_str(" yeah"); // push_str() takes a string slice
        s.push('!'); // push() takes a single character
        println!("{}", s); // output: yeah yeah!
    }
    {
        let s1 = String::from("Hello");
        let s2 = String::from(", How are you?");
        let s3: String = s1 + &s2;
        // format macro
        let s4: String = format!("{}-{}", s3, s2);
        // println!("s1: {}", s1); // error
        println!("s2: {}\ns3: {}", s2, s3);
        println!("s4: {}", s4);
    }
    {
        let hello: String = String::from("你好");
        /* Bytes:
        [228, 189, 160, 229, 165, 189]
        [11100100, 10111101, 10100000, 11100101, 10100101, 10111101]

        Scalar values:
        ['你', '好']  */

        // let c: char = hello[0]; // it won't work cause the index is not a char boundary

        // iterate over "bytes"
        print!("\nthrough bytes:\t\t");
        for byte in hello.bytes() {
            print!("{} ", byte);
        }

        // iterate over "chars"
        print!("\nthrough chars:\t\t");
        for char in hello.chars() {
            print!("{}", char);
        }

        // iterate over "grapheme clusters"
        print!("\nthrough graphemes:\t");
        for grapheme in hello.graphemes(true) {
            print!("{}", grapheme);
        }
    }
}
