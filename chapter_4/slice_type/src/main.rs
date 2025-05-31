// Slices have a pointer to beginning of data and stores the length of the slice.
// Can only be used for immutable references.

fn main() {
    {
        let s = String::from("Hello, world!");

        println!("s: {}", s);

        let s1 = &s[..6];
        let s2 = &s[6..];

        println!("s1: '{}', s2: '{}'", s1, s2);
    }

    {
        let mut s = String::from("Hello, world!");

        println!("s: {}", s);

        let first_word = first_word(&s);

        println!("First word: '{}'", first_word);
    }

    {
        let my_string = String::from("hello world");

        let word = first_word(&my_string[0..6]);
        let word = first_word(&my_string[..]);
        let word = first_word(&my_string);

        let my_string_literal = "Hello, world!";

        let word = first_word(&my_string_literal[0..6]);
        let word = first_word(&my_string_literal[..]);
        let word = first_word(&my_string_literal);
    }

    {
        let a = [1, 2, 3, 4, 5];

        let slice = &a[1..3];

        assert_eq!(slice, &[2, 3]);
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    s
}
