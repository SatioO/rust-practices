fn main() {
    let mut a: Vec<i8> = vec![1, 2, 3, 4];
    {
        let b = &mut a;
        b.push(5);
    }
    println!("{:?}", a);
}
