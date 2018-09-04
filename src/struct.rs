#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    status: bool,
}

fn main() {
    let user1 = User {
        name: String::from("Vaibhav"),
        age: 27,
        status: true,
    };

    let user2 = build_user(user1);
    println!("{}", user2);
    // println!("name: {}, age: {}", user1.name, user1.age);
}

fn build_user(user: User) -> String {
    user.name
}
