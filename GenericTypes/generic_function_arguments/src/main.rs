fn main() {
    let list = vec![34, 50, 25, 100, 65];
    let result = get_largest(list);
    println!("The largest number is {}", result); // output: 100

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = get_largest(char_list);
    println!("The largest char is {}", result); // output: y
}

fn get_largest<T: PartialOrd + Copy>(number_list: Vec<T>) -> T {
    // T for Type
    let mut largest = number_list[0];
    for number in number_list {
        if number > largest {
            largest = number;
        }
    }
    largest
}
