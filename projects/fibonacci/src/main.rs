fn fibonacci(n: u32) -> u64 {
    let (mut a, mut b) = (0, 1);

    for _ in 0..n {
        let temp = a;
        a = b;
        b = temp + b;
    }

    a
}

fn main() {
    let n = 10;
    println!("The {n}th Fibonacci number is: {}", fibonacci(n));
}