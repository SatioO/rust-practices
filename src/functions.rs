fn main() {
    let result = add(5, 6);
    println!("result is {}", result)
}

fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

struct User {
    name: String,
    email: String,
    age: i32,
    active: bool,
}

fn build_user(name: String, email: String, age: i32, active: bool) -> User {
    return {
        User {
            name: name,
            email: email,
            age: age,
            active: active,
        }
    };
}

build_user("Vaibhav Satam", "vaibhav.satam29991@gmail.com", 32, true)
