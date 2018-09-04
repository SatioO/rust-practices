fn main() {
    let mut ab = String::from("Vaibhav");
    // Referencing and borrowing
    {
        let ac = &mut ab; // mutable reference
        ac.push('a'); //mutating values
        println!("ac: {}", ac);
    }
    ab.push('s');

    // Tranferring Ownership
    foo(&ab);
    println!("ab: {}", ab);
}

fn foo(val: &String) {
    println!("value: {}", val);
}
