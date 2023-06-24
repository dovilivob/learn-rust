// use <T, U> to define multiple generic types
struct TwoTypePoint<T, U> {
    x: T,
    y: U,
}
impl<T, U> TwoTypePoint<T, U> {
    // this mixup() method mix point a (x: T, y: U) with point b (x: V, y: W)
    // and take the x of point a and the y of point b to create a new point
    // so the return type should be TwoTypePoint<T, W>
    fn mixup<V, W>(self, other: TwoTypePoint<V, W>) -> TwoTypePoint<T, W> {
        TwoTypePoint {
            x: self.x,
            y: other.y,
        }
    }
}

struct OneTypePoint<T> {
    x: T,
    y: T,
}

impl<T> OneTypePoint<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl OneTypePoint<f64> {
    fn y(&self) -> f64 {
        self.y
    }
}

fn main() {
    {
        let p_1 = TwoTypePoint { x: 5, y: 10 }; // this only work when we use <i32>
        let p_2 = TwoTypePoint { x: 5.0, y: 10.0 }; // this won't work if we use <i32> instead of <T> or <T, U>
        let p_3 = TwoTypePoint { x: 5, y: 10.0 }; // this won't work if we use <T> instead of <T, U>
    }
    {
        let p_1 = OneTypePoint { x: 5, y: 10 };
        p_1.x(); // can't use p_1.y() because y is not defined for i32
        let p_2 = OneTypePoint { x: 5.0, y: 10.0 };
        p_2.x();
        p_2.y(); // p_2.y() is allowed because y is defined for f64
    }
    {
        let p_1 = TwoTypePoint { x: 5, y: 10.4 };
        let p_2 = TwoTypePoint { x: "Hello", y: 'c' };
        let p_3 = p_1.mixup(p_2);
        println!("p_3.x = {}, p_3.y = {}", p_3.x, p_3.y);
    }
}
