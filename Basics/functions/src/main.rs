fn main() {
    println!("Hello, world!");
    print_sum_of(1, 2);

    let y = {
        let x = 3;
        x + 1 // no semicolon
    };
    println!("The value of y is: {y}"); // output: 4

    println!("The value of five() is: {}", five());
    println!("The value of plus_one(10) is: {}", plus_one(10));
}

// Functions
fn print_sum_of(x: i32, y: i32) {
    println!("The sum of {} and {} is {}", x, y, x + y);
}

// functions with return value
fn five() -> i32 {
    5 // no semicolon
}

fn plus_one(x: i32) -> i32 {
    x + 1 // no semicolon
}
