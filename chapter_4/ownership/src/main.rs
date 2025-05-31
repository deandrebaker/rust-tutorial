// Ownership rules
// 1. Each value in Rust has an owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

// Benefits:
// 1. Memory safety without garbage collection
// 2. Avoids double free errors
// 3. Automatic copying of data is inexpensive (only shallow)

fn main() {
    {
        // s is invalid here and before this point
        let s = "hello"; // s is valid
        println!("s is: {}", s);
    } // s is invalid here and after this point

    {
        let a = String::from("hello"); // The string is allocated on the heap
        println!("a is: {}", a);
    } // a is invalid (out of scope). Since a is the owner of the string, the string is dropped
    // here.

    {
        let mut b = String::from("hello");
        println!("b is: {}", b);
        b.push_str(", world");
        println!("b is: {}", b);
    }

    {
        let x = 5;
        let y = x; // x is copied to y because it is on the stack. This is from the Copy trait.
        // integers, booleans, floating-point numbers, characters, and tuples of only the types
        // mentioned above implement the Copy trait.
        println!("x is: {}, y is: {}", x, y);
    }

    {
        let s1 = String::from("hello");
        let s2 = s1;
        // s1 is no longer valid here because ownership has been moved to s2
        // This is to prevent a double free error
        // println!("s1 is: {}, s2 is: {}", s1, s2);
        println!("s2 is: {}", s2);
    }

    {
        let mut s = String::from("hello"); // "hello" is allocated
        println!("s is: {}", s);
        s = String::from("ahoy"); // "hello" is dropped and "ahoy" is allocated
        println!("s is: {}", s);
    } // "ahoy" is dropped

    {
        let s1 = String::from("hello");
        let s2 = s1.clone(); // Performs a deep copy of the string. Ownership isn't moved
        println!("s1 is: {}, s2 is: {}", s1, s2);
    }

    {
        let s = String::from("hello");
        println!("s is: {}", s);
        take_ownership(s); // Ownership of s is moved to the function
        // println!("s is: {}", s); // This line would cause a compile error because s is no longer
        // valid

        let x = 5;
        println!("x is: {}", x);
        makes_copy(x); // x is copied, so it is still valid after this function call
        println!("x is still valid: {}", x); // This line is valid because x is copied
    }

    {
        let s1 = give_ownership();
        println!("s1 is: {}", s1);

        let s2 = String::from("hello");
        println!("s2 is: {}", s2);
        let s3 = take_and_give_back_ownership(s2);
        println!("s3 is: {}", s3);
    }

    {
        let s1 = String::from("hello");
        let (s2, len) = calculate_length(s1);
        println!("The length of '{}' is {}.", s2, len);
    }
}

fn take_ownership(some_string: String) {
    println!("Taking ownership of: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("Making a copy of: {}", some_integer);
}

fn give_ownership() -> String {
    let some_string = String::from("yours");
    println!("Giving ownership of: {}", some_string);
    some_string
}

fn take_and_give_back_ownership(some_string: String) -> String {
    println!("Taking ownership of: {}", some_string);
    some_string
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}
