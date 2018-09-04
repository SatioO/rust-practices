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
    foo(&mut ab);
    ab.push('s');
    println!("ab: {}", ab);

    //
    let count = 3;
    let count1 = count;
    take(count);
    println!("count: {}, count1: {}", count, count1);
}

fn take(val: i32) {
    println!("count: {}", val);
}

fn foo(val: &mut String) {
    println!("value: {}", val);
    val.push('s');
}
