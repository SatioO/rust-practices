fn main() {
    let optional = Some(7);

    match optional {
        Some(i) => println!("{}", i),
        None => {}
    }

    if let Some(i) = optional {
        println!("{}", i);
    }
}
