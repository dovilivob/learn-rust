use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    // use match expression
    let _f = match f {
        Ok(file) => file,
        // Err(error) => panic!("Problem opening the file: {:?}", error),
        Err(error) => match error.kind() {
            // if the error is NotFound, create the file, but this operation can still fail, so we need to handle the result
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => {
                    println!("File {:?} Created!", &fc);
                    fc
                }
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    // use closure to simplify the code
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // however, you can simply use unwrap() or expect() in most cases
    let _f = File::open("hello.txt").unwrap(); // the return type of _f is File, not Result<File, Error>
    let _f = File::open("hello.txt").expect("Failed to open hello.txt"); // use expect to provide a custom error message in one line
}
