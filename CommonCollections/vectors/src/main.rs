fn main() {
    let _a: [u32; 3] = [4, 2, 0]; // use prefix "_" to avoid warning about unused variable

    let mut v: Vec<u32> = Vec::new(); // create an empty vector

    v.push(4);
    v.push(2);
    v.push(0);

    println!("v: {:?}", v);

    {
        let mut v2: Vec<u32> = vec![4, 2, 0, 6, 9]; // create a vector with initial values
        v2.push(20); // this create a mutable reference to the vector, but won't be alive after this line, so the next line is allowed
        let third_item: &u32 = &v2[2]; // get a reference to the third item

        // v2.push(20); // to put this line here is not allowed because it create two mutable references to the same vector

        // let twentieth_item: &u32 = &v2[19]; // get a reference to the third item
        println!("The third item is {}", third_item);
        // println!("The twentieth item is {}", twentieth_item); // it causes a runtime error

        /* when using array, it causes a "compile" error
        but when using vector, it causes a "runtime" error */

        // use get() method to get a reference to the third item, which prevent runtime error

        let mut item_index = 19;

        match v2.get(item_index) {
            Some(item) => println!("The third item is {}", item),
            None => println!("There is no No. {} item", item_index),
        }
        // output: There is no No. 19 item

        item_index = 2;

        match v2.get(item_index) {
            Some(item) => println!("The third item is {}", item),
            None => println!("There is no No. {} item", item_index),
        }
        // output: The third item is 0
    }

    {
        let immut_v = vec![4, 2, 0, 6, 9];
        // loop through "immutable" vector
        for i in &immut_v {
            println!("{}", i);
        }

        println!("immut_v: {:?}", immut_v); // output: immut_v: [4, 2, 0, 6, 9]

        // loop through "mutable" vector
        let mut mut_v = vec![3, 1, -1, 5, 8];
        for i in &mut mut_v {
            *i += 1; // use "*" to dereference the value
            println!("{}", i);
        }
        println!("mut_v: {:?}", mut_v); // output: mut_v: [4, 2, 0, 6, 9]
    }

    // by default, vectors only store one type. use enum to store multiple types in a vector.
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        // it's ok to put these types in different order
        SpreadsheetCell::Int(42),
        SpreadsheetCell::Text(String::from("Hello")),
        SpreadsheetCell::Float(0.69),
    ];

    let item_index = 8;
    match row.get(item_index) {
        Some(SpreadsheetCell::Int(i)) => println!("The item of {} is {}", item_index, i),
        Some(SpreadsheetCell::Float(f)) => println!("The item of {} is {}", item_index, f),
        Some(SpreadsheetCell::Text(t)) => println!("The item {} is {}", item_index, t),
        _ => println!("There is no No. {} item", item_index),
    }
}
