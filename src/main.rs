#![allow(dead_code)]

#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Right(Point),
    Left(Point),
}

#[derive(Debug)]
enum Keys {
    UpKey,
    DownKey,
    LeftKey,
    RightKey,
}

#[derive(Debug)]
struct Point {
    x: u32,
    y: u32,
}

fn main() {
    let point = Point { x: 30, y: 40 };
    let direction = Direction::Down(Point { x: 30, y: 40 });

    let key = Keys::RightKey;
    let pressed = match key {
        Keys::UpKey => Direction::Up(point),
        Keys::DownKey => Direction::Down(point),
        Keys::RightKey => Direction::Right(point),
        Keys::LeftKey => Direction::Left(point),
    };
    println!("pressed key: {:?}", pressed);

    // matching expression
    match &direction {
        Direction::Up(point) => println!("{:?}", point),
        Direction::Down(point) => println!("{:?}", point),
        Direction::Right(point) => println!("{:?}", point),
        Direction::Left(point) => println!("{:?}", point),
    }

    // returning from match expression
    let points: &Point = match &direction {
        Direction::Up(point) => point,
        Direction::Down(point) => point,
        Direction::Right(point) => point,
        Direction::Left(point) => point,
    };
    println!("{:?}", points);
}
