struct Point {
    x: f64,
    y: f64,
}

impl Point {
    // static method without params
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }
    // static method with params
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

fn main() {
    let origin = Point::origin();
    let point = Point::new(origin.x, origin.y);
    println!("X: {:?}, Y: {:?}", point.x, point.y);
}
