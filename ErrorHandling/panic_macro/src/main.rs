fn main() {
    a();
}

fn a() {
    b();
}

fn b() {
    c(0);
}

fn c(num: i32) {
    if num == 0 {
        panic!("c is zero, no one can pass 0 to this function, panic!");
    }
}
