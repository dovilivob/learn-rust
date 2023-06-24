fn main() {
    let r;
    {
        let x = 5;
        r = &x;
        println!("r: {}", r); // this work
    }
    // println!("r: {}", r); // this won't work because x is out of scope, and that cause dangling reference
}
