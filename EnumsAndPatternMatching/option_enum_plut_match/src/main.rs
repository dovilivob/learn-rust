fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("six: {}, none: {:?}", six.unwrap(), none);

    let some_value = Some(5);
    match some_value {
        Some(3) => println!("{} is three", some_value.unwrap()),
        _ => println!("{:?} is not three", some_value),
    }

    // if let is a shortcut for match that only handles one case
    if let Some(3) = some_value {
        println!("{} is three", some_value.unwrap());
    } else {
        println!("{:?} is not three", some_value);
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    // Use Some() because our return value is optional
    match x {
        // None => None,
        Some(i) => Some(i + 1),
        _ => None,
    }
}
