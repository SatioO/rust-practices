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

fn mul<F>(num: i32, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    f(num)
}

fn main() {
    // closure without params
    fn pr() {
        println!("called normal function")
    }

    let p = || println!("called after run");
    run(p);
    run(pr);

    // closure with params
    let x = |x| x * x;
    let result = add(x);
    println!("result: {:?}", result);

    // closure with params
    let y = |x| x + 10;
    let num = 10;
    let result = mul(num, y);
    println!("result: {:?}", result)
}
