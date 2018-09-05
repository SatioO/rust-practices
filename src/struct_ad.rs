#[derive(Debug)]
struct Object {
    width: i32,
    height: i32,
}

// implementation methods
impl Object {
    fn area(&self) -> i32 {
        self.height * self.width
    }
}

// static methods
impl Object {
    fn new(width: i32, height: i32) -> Object {
        Object { width, height }
    }
}

fn main() {
    let o1 = Object {
        width: 30,
        height: 30,
    };

    let o2 = Object::new(40, 30);

    println!(
        "height x width: {} x {} = {:#?}",
        o1.height,
        o1.width,
        o1.area()
    );
    println!(
        "height x width: {} x {} = {:#?}",
        o2.height,
        o2.width,
        o2.area()
    );
}
