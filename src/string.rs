fn main() {
    let hello = String::from("Hello world !");
    let hello1 = "Hello world !";
    print_str(&hello);
    print_str(hello1);
}

fn print_str(val: &str) {
    println!("{}", val)
}
