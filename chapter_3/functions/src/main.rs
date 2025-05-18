fn main() {
    println!("Hello, world!");

    another_function(5);
    print_labeled_measurement(5, 'h');

    let y = {
        let x = get_three();
        add_one(x)
    };

    println!("The value of y is: {y}");
}

fn another_function(x: i32) {
    println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn get_three() -> i32 {
    3
}

fn add_one(x: i32) -> i32 {
    x + 1
}
