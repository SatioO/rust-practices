fn main() {
    let age = 17;

    let can_vote: Option<bool> = if age > 18 { Some(true) } else { None };

    let result: bool = match can_vote {
        Some(value) => value,
        None => false,
    };

    println!("{}", result);
}
