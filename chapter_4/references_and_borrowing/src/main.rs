// Borrrowing rules
// 1. Owners can have references to their data that allow them to borrow but not own it.
// 2. At any given time, you can have either one mutable reference or any number of immutable
//    references.
// 3. References must always be valid.

// Benefits:
// 1. Prevents data races
// 2. Prevents dangling references

fn main() {
    {
        let s = String::from("Hello, world!");

        let len = calculate_length(&s); // "Hello, world!" is borrowed

        println!("The length of '{s}' is {len}.");
    }

    {
        let mut s = String::from("hello");

        append_world(&mut s);

        println!("{}", s);
    }

    {
        let mut s = String::from("hello");

        println!("s: {}", s);

        let mut r1 = &mut s;
        println!("r1: {}", r1);

        append_world(&mut r1);

        println!("r1: {}", r1);

        println!("s: {}", s);

        append_world(&mut s);

        // println!("r1: {}", r1);

        println!("s: {}", s);
    }

    {
        let mut s = String::from("hello");

        println!("s: {}", s);

        let mut r1 = &mut s;
        println!("r1: {}", r1);

        r1.push_str(", world");

        println!("r1: {}", r1);

        println!("s: {}", s);
        s.push_str(", world");

        // println!("r1: {}", r1);

        println!("s: {}", s);
    }

    {
        let mut s = String::from("hello");

        {
            let r1 = &mut s;
        }

        let r2 = &mut s;
    }

    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;

        println!("r1: {}, r2: {}", r1, r2);
    }

    {
        let mut s = String::from("hello");

        let r1 = &s;
        let r2 = &s;
        println!("{} and {}", r1, r2);

        let r3 = &mut s;
        println!("{}", r3);
    }

    {
        let s = no_dangle();

        println!("s: {}", s);
    }
}

// s is a reference to a String and doesn't take ownership
fn calculate_length(s: &String) -> usize {
    // s is an immutable reference to a String and cannot be modified
    s.len()
}

fn append_world(s: &mut String) {
    s.push_str(", world");
}

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
