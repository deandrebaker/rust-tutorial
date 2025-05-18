use std::io;

fn main() {
    println!("Please input an integer.");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line from input.");

    let number: i32 = input
        .trim()
        .parse()
        .expect("Failed to parse input '{input}' as a i32.");

    if number < 5 {
        println!("condition was true.");
    } else {
        println!("condition was false.");
    }

    let msg = if number % 4 == 0 {
        "number is divisible by 4."
    } else if number % 3 == 0 {
        "number is divisible by 3"
    } else if number % 2 == 0 {
        "number is divisible by 2"
    } else {
        "number is not divisible y 4, 3, or 2."
    };

    println!("{msg}");

    let mut counter = 0;

    let counted_number = loop {
        counter += 1;
        println!("counter: {counter}");

        if counter == number {
            break counter;
        }
    };

    println!("Counted number: {counted_number}");

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = number;

        loop {
            println!("remaining = {remaining}");
            if remaining == 0 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");

    let mut number = number;

    while number != 0 {
        println!("number = {number}");
        number -= 1;
    }

    println!("Exited while loop");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    for element in a {
        println!("the value is: {element}");
    }

    for element in (1..4).rev() {
        println!("the value is: {element}");
    }
}
