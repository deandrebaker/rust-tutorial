use std::io;

fn main() {
    // Enter temperature amount
    let mut temp_amount = String::new();
    println!("Please enter the temperature amount as a number");

    io::stdin()
        .read_line(&mut temp_amount)
        .expect("Failed to read line from standard input");

    let temp_amount: f64 = temp_amount
        .trim()
        .parse()
        .expect("Failed to parse temp_amount '{temp_amount}'");

    // Enter temperature unit
    let mut temp_unit = String::new();
    println!("Please enter the temperature unit as either 'f' or 'c'");

    io::stdin()
        .read_line(&mut temp_unit)
        .expect("Failed to read line from standard input");

    let temp_unit = temp_unit.trim().to_lowercase();

    // Return converted temperature
    let (converted_temp_amount, converted_temp_unit) = match temp_unit.as_str() {
        "f" => (convert_f_to_c(temp_amount), "c"),
        "c" => (convert_c_to_f(temp_amount), "f"),
        _ => panic!("Temperature unit was invalid"),
    };

    println!("Converted temperature: {converted_temp_amount} {converted_temp_unit}");
}

fn convert_f_to_c(t: f64) -> f64 {
    (t - 32.0) * 5.0 / 9.0
}

fn convert_c_to_f(t: f64) -> f64 {
    t * 9.0 / 5.0 + 32.0
}

#[test]
fn test_convert_f_to_c() {
    assert_eq!(0.0, convert_f_to_c(32.0));
    assert_eq!(100.0, convert_f_to_c(212.0));
    assert_eq!(37.0, convert_f_to_c(98.6));
    assert_eq!(-17.7777, (convert_f_to_c(0.0) * 10000.0).trunc() / 10000.0);
    assert_eq!(-40.0, convert_f_to_c(-40.0));
}

#[test]
fn test_convert_c_to_f() {
    assert_eq!(32.0, convert_c_to_f(0.0));
    assert_eq!(212.0, convert_c_to_f(100.0,));
    assert_eq!(98.6, convert_c_to_f(37.0,));
    assert_eq!(
        0.0001,
        (convert_c_to_f(-17.7777) * 10000.0).trunc() / 10000.0
    );
    assert_eq!(-40.0, convert_c_to_f(-40.0,));
}
