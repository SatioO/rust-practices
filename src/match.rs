#[derive(Debug)]
enum Direction {
    Up(Point),
    Down(Point),
    Right(Point),
    Left(Point),
}

#[derive(Debug)]
enum Keys {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 30, y: 40 };
    let direction = Direction::Down(point);

    let key = Keys::Up;
    let pressed = match key {
        Up => Up,
        Down => Down,
        Right => Right,
        Left => Left,
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
