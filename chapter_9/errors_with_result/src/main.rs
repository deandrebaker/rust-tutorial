use std::error::Error;
use std::fs::{self, File};
use std::io::{self, ErrorKind, Read};

fn main() -> Result<(), Box<dyn Error>> {
    if false {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(err) => panic!("Problem opening the file: {err:?}"),
        };
    }

    if false {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(err) => match err.kind() {
                ErrorKind::NotFound => match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!("Problem create the file: {e:?}"),
                },
                other_error => panic!("Problem opening the file: {other_error:?}"),
            },
        };
    }

    if false {
        let greeting_file = File::open("hello.txt").unwrap_or_else(|err| {
            if err.kind() == ErrorKind::NotFound {
                File::create("hello.txt")
                    .unwrap_or_else(|err| panic!("Problem create the file: {err:?}"))
            } else {
                panic!("Problem opening the file: {err:?}")
            }
        });
    }

    if false {
        let greeting_file = File::open("hello.txt").unwrap();
    }

    if false {
        let greeting_file =
            File::open("hello.txt").expect("hello.txt should be included in this project");
    }

    if false {
        match read_username_from_file() {
            Ok(username) => println!("Username: {username}"),
            Err(e) => panic!("Problem reading the file: {e:?}"),
        }
    }

    if false {
        match read_username_from_file2() {
            Ok(username) => println!("Username: {username}"),
            Err(e) => panic!("Problem reading the file: {e:?}"),
        }
    }

    // {
    //     let greeting_file = fs::read_to_string("hello.txt");
    // }

    if false {
        let c = last_char_of_first_line("Hello\nWorld");
    }

    if true {
        let greeting_file = File::open("hello.txt")?;
    }

    Ok(())
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_from_file2() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
