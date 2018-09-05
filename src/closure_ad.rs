fn run<F>(f: F)
where
    F: Fn(),
{
    println!("Inside run function");
    f();
}

fn add<F>(f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(3)
}

fn main() {
    // closure without params
    let f = || println!("called after run");
    run(f);

    // closure with params
    let x = |x| x + 10;
    let result = add(x);
    println!("result: {:?}", result)
}
