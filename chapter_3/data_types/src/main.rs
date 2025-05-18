use std::io;

fn main() {
    let a: i8 = -128;
    let b: u8 = 127;
    let c: i16 = -(1 << 14) - 1;
    let d: u16 = (1 << 15) - 1;
    let e: i32 = -(1 << 30) - 1;
    let f: u32 = (1 << 31) - 1;
    let g: i64 = -(1 << 62) - 1;
    let h: u64 = (1 << 63) - 1;
    let i: i128 = -(1 << 126) - 1;
    let j: u128 = (1 << 127) - 1;
    let k: isize = -(1 << 62) - 1;
    let l: usize = (1 << 63) - 1;

    println!("Hello");
    println!("{a}");
    println!("{b}");
    println!("{c}");
    println!("{d}");
    println!("{e}");
    println!("{f}");
    println!("{g}");
    println!("{h}");
    println!("{i}");
    println!("{j}");
    println!("{k}");
    println!("{l}");

    let dec = 98_222;
    let hex = 0xff;
    let oct = 0o77;
    let bin = 0b1111_0000;
    let byt = b'a';

    println!("{dec}");
    println!("{hex}");
    println!("{oct}");
    println!("{bin}");
    println!("{byt}");

    let m: f32 = 13.55;
    let n: f64 = 2.3;

    println!("{m}");
    println!("{n}");

    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("{sum}");
    println!("{difference}");
    println!("{product}");
    println!("{quotient}");
    println!("{truncated}");
    println!("{remainder}");

    let tr: bool = true;
    let fa: bool = false;

    println!("{tr}");
    println!("{fa}");

    let lowercase_char: char = 'z';
    let uppercase_char: char = 'Z';
    let heart_eyed_cat: char = '😻';

    println!("{lowercase_char}");
    println!("{uppercase_char}");
    println!("{heart_eyed_cat}");

    let tup: (i32, f64, bool) = (-400, 3.2, false);
    let (x, y, z) = tup;
    let xx = tup.0;
    let yy = tup.1;
    let zz = tup.2;

    println!("{tup:?}");
    println!("{x}");
    println!("{y}");
    println!("{z}");
    println!("{xx}");
    println!("{yy}");
    println!("{zz}");

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr2 = [7; 3];
    let first = arr[0];
    let last = arr[4];

    println!("{arr:?}");
    println!("{arr2:?}");
    println!("{first}");
    println!("{last}");

    println!("Please enter an array index for {arr:?}");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = arr[index];

    println!("The value of the element at index {index} is: {element}");
}
