use std::io;

fn main() {
    let mut number = String::new();
    println!("Please enter the temperature amount as a number");

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line from standard input");

    let number: u32 = number
        .trim()
        .parse()
        .expect("Failed to parse number '{number}'");

    let result = fib(number);
    println!("Fibonacci number at index {number}: {result}");
}

fn fib(n: u32) -> u32 {
    if n < 2 { n } else { fib(n - 1) + fib(n - 2) }
}
