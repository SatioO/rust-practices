fn run<F>(f: F)
where
    F: Fn(),
{
    println!("Inside run function");
    f();
}

fn main() {
    let f = || println!("hello world");
    run(f);
}
