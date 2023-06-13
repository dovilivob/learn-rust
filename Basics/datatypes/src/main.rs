fn main() {
    // Floating-Point Types
    let x = 42.0; // f64
    let y: f32 = 6.9; // f32

    // Numeric Operations
    let sum = 420 + 69;
    let difference = 42.0 - 6.9;
    let product = 4 * 20;
    let quotient = 4.2 / 0.69;
    let truncated = -420 / 69; // Result is -6, not -7
    let remainer = 420 % 69;

    // The Boolean Type
    let t = true;
    let f: bool = false; // with explicit type annotation

    // The Character Type
    // Use single quotes, not double quotes
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';

    // Compound Types
    // The Tuple Type
    let tup: (i32, f64, u8) = (420, 6.9, 1);
    let (x, y, z) = tup; // destructuring
    println!("The value of y is: {}", y);

    // Accessing tuple elements directly by using a period (.) followed by the index of the value we want to access
    let tup_0 = tup.0;
    let tup_1 = tup.1;
    let tup_2 = tup.2;
    println!("numbers in tuple: {}, {}, {}", tup_0, tup_1, tup_2);

    // The Array Type
    let a = [4, 2, 0, 6, 9];
    let a: [i32; 5] = [4, 2, 0, 6, 9];
    println!("array: {:?}", a); // use {:?} to print array
    let a = [3; 5]; // same as let a = [3, 3, 3, 3, 3];
    println!("array: {:#?}", a); // use {:#?} to print array with pretty print

    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    // Array Indexing
    let jan = months[0];
    println!("The first month is: {}", jan);
}
