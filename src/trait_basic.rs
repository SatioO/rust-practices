trait Shape {
    fn area(&self) -> f32;
}

struct Rectangle {
    x: f32,
    y: f32,
}

impl Shape for Rectangle {
    fn area(&self) -> f32 {
        self.x * self.y
    }
}

fn main() {
    let rect = Rectangle { x: 30.0, y: 40.0 };
    let area = rect.area();
    println!("area: {:?}", area);
}
