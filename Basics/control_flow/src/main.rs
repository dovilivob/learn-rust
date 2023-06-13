fn main() {
    let num = 7;
    if num < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    // You can't use if num { ... } like in C/C++ or JS or other languages

    let num = 20;
    if num % 4 == 0 {
        println!("num is {num} and is divisible by 4");
    } else if num % 3 == 0 {
        println!("num is {num} and is divisible by 3");
    } else if num % 2 == 0 {
        println!("num is {num} and is divisible by 2");
    } else {
        println!("num is {num} and is not divisible by 4, 3, or 2");
    }

    // Using if in a let statement
    let condition = true;
    // it's like ? : in C/C++
    let num = if condition { 5 } else { 6 };
    // Notice that the types of the blocks need to be the same
    // It's can't be like this:
    // let num = if condition { 5 } else { "six" };
    println!("The value of num is: {num}");

    // Looping
    let mut counter = 0;
    let result = loop {
        counter += 1;
        println!("counter = {counter}");
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    println!("\n// Nested Loop");
    nested_loop();
    println!("\n// While loop");
    while_loop();
    println!("\n// Loop through a collection");
    loop_through_collection();
}

// Loop labels to disambiguate between multiple nested loops
fn nested_loop() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 3;

        loop {
            println!("remaining = {remaining}");
            if remaining == 0 {
                break;
            }
            if count == 10 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }

    println!("End count = {count}");
}

// Conditional Loops with while
fn while_loop() {
    let mut num = 3;
    while num != 0 {
        println!("num = {num}");
        num -= 1;
    }
    println!("End num = {num}");
}


// Loop through a collection with for 
fn loop_through_collection () {
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("element = {}", element);
    }
    // You can also use a range
    for number in (1..4).rev() { // rev() reverses the range
        println!("number = {number}");
    }
}