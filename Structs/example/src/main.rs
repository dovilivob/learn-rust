#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Method implementation
impl Rectangle {
    // Pass by reference so that the function does not take ownership of the struct
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Multiple impl blocks is allowed
impl Rectangle {
    // Associated function, not method, because it does not take self as a parameter
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let width: u32 = 30;
    let height: u32 = 50;

    let rect_1: Rectangle = Rectangle {
        width: width,
        height: height,
    };

    let rect_2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };

    let rect_3: Rectangle = Rectangle {
        width: 60,
        height: 45,
    };

    // User :: instead of . when calling associated function
    let square_1: Rectangle = Rectangle::square(3);

    println!("rect: {:#?}", rect_1);
    println!(
        "Through method in struct: {}",
        rect_1.area()
    );
    // Use . to call method because the built-in automatic referencing and dereferencing feature
    println!("Can rect_1 hold rect_2? {}", rect_1.can_hold(&rect_2)); // return true
    println!("Can rect_1 hold rect_3? {}", rect_1.can_hold(&rect_3)); // return false
    println!("Can rect_1 hold square_1? {}", rect_1.can_hold(&square_1)); // return true
}