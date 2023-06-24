use std::fs::{self, File};
use std::io::{self, Read};

fn main() {
    let user_name = read_username_from_file().unwrap();
    println!("user name: {}", user_name);
}

// this function return a String or an Error
fn read_username_from_file() -> Result<String, io::Error> {
    // fs::read_to_string("hello.txt") // ultra simplified, this is the same as the following code

    let mut s = String::new();
    // let mut f = File::open("hello.txt")?;
    // f.read_to_string(&mut s)?;
    File::open("hello.txt")?.read_to_string(&mut s)?; // super simplified
    Ok(s)
    // let f = File::open("hello.txt");

    // this is too long, use the ? operator to simplify it
    // let mut f = match f {
    //     Ok(file) => file,
    //     Err(error) => return Err(error),
    // };

    // so does this chunk of code
    // match f.read_to_string(&mut s) {
    //     Ok(_) => Ok(s),
    //     Err(error) => Err(error),
    // }
}
