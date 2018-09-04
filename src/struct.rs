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

    let user2 = build_user(&user1);
    println!(
        "name: {}, age: {}, status: {}",
        user2.name, user2.age, user2.status
    );

    println!(
        "name: {}, age: {}, status: {}",
        user1.name, user1.age, user1.status
    );

    let User { name, age, status } = user1;
    println!("name: {}, age: {}, status: {}", name, age, status);
}

fn build_user(user: &User) -> User {
    User {
        name: String::from("Rohit"),
        age: user.age,
        status: user.status,
    }
}
