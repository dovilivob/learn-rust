fn main() {
    // make 'x' mutable by add "mut"
    let mut x = 5;
    println!("The Value of x is: {x}");
    x = 10;
    println!("The Value of x is: {x}");

    // you can't add "mut" to const, it's always immutable
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let y = 5;
    println!("the Value of y is: {y}");
    let y = y + 1;
    println!("the Value of y is: {y}");

    // Scope
    {
        let x = x * 2;
        println!("the Value of x in the inner scope is: {x}");
    }
    println!("the Value of x in the outer scope is: {x}");

    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {spaces}");

    // You can't do this because the type of variable can't be changed, even if it's mutable.
    // let mut spaces = "   ";
    // spaces = spaces.len();
    // println!("spaces: {spaces}");
}
