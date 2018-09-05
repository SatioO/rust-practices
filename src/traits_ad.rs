use std::ops::Mul;

trait Shape<T: Mul> {
    fn area(&self) -> T;
}

struct Rectangle<T: Mul> {
    x: T,
    y: T,
}

impl<T: Copy> Shape<T> for Rectangle<T>
where
    T: Mul<Output = T>,
{
    fn area(&self) -> T {
        self.x * self.y
    }
}

fn main() {
    let rect = Rectangle { x: 30.0, y: 40.0 };
    let area = rect.area();
    println!("area: {:?}", area);
}
