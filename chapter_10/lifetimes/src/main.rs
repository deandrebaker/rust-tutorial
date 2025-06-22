// Elision rules
// 1. Assign a lifetime parameter to each parameter that's a reference.
// 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output
//    lifetime parameters.
// 3. If there are multiple input lifetime parameters but only one of them is &self or &mut self
//    because this is a method, the lifetime of self is assigned to all output lifetime parameters.

fn main() {
    // {
    //     let r;
    //
    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    //
    //     println!("r: {r}");
    // }

    {
        let x = 5;
        let r = &x;
        println!("r: {r}");
    }

    {
        let string1 = String::from("abcd");
        let string2 = "xyz";
        let result = longest(string1.as_str(), string2);
        println!("The longest string is: {result}");
    }

    // {
    //     let string2 = "xyz";
    //     let result;
    //
    //     {
    //         let string1 = String::from("abcd");
    //         result = longest(string1.as_str(), string2);
    //     }
    //
    //     println!("The longest string is: {result}");
    // }

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        println!("Important excerpt: {:?}", i);
    }

    // {
    //     let i;
    //     {
    //         let novel = String::from("Call me Ishmael. Some years ago...");
    //         let first_sentence = novel.split('.').next().unwrap();
    //         i = ImportantExcerpt {
    //             part: first_sentence,
    //         };
    //     }
    //     println!("Important excerpt: {:?}", i);
    // }

    {
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().unwrap();
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        println!(
            "Important excerpt: {:?}",
            i.announce_and_return_part("This is an announcement!")
        );
    }

    {
        let string1 = String::from("abcd");
        let result;

        {
            let string2: &'static str = "xyz";
            result = longest(string1.as_str(), string2);
        }

        println!("The longest string is: {result}");
    }

    {
        let string1 = String::from("abcd");
        let string2 = "xyz";
        let ann = Annoucement {
            message: String::from("This is an announcement!"),
        };
        let result = longest_with_an_announcement(string1.as_str(), string2, ann);
        println!("The longest string is: {result}");
    }
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

// fn longest2<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }

#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[allow(clippy::needless_lifetimes)]
impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention! {announcement}");
        self.part
    }
}

use std::fmt::Display;

struct Annoucement {
    message: String,
}

impl Display for Annoucement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {ann}");
    if x.len() > y.len() { x } else { y }
}
