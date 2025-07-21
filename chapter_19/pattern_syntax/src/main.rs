fn main() {
    {
        let x = 1;
        match x {
            1 => println!("one"),
            2 => println!("two"),
            3 => println!("three"),

            _ => println!("anything"),
        }
    }

    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(y) => println!("Matched, y = {y}"),
            _ => println!("Default case, x = {x:?}"),
        }

        println!("at the end: x = {x:?}, y = {y}");
    }

    {
        let x = 1;
        match x {
            1 | 2 => println!("one or two"),
            3 => println!("three"),
            _ => println!("anything"),
        }
    }

    {
        let x = 5;
        match x {
            1..=5 => println!("one through five"),
            _ => println!("something else"),
        }
    }

    {
        let x = 'c';
        match x {
            'a'..='j' => println!("early ASCII letter"),
            'k'..='z' => println!("late ASCII letter"),
            _ => println!("something else"),
        }
    }

    {
        let p = Point { x: 0, y: 7 };
        let Point { x: a, y: b } = p;
        println!("p.x = {a}, p.y = {b}");
    }

    {
        let p = Point { x: 0, y: 7 };
        let Point { x, y } = p;
        println!("p.x = {x}, p.y = {y}");
    }

    {
        let p = Point { x: 0, y: 7 };

        match p {
            Point { x, y: 0 } => println!("On the x-axis at {x}"),
            Point { x: 0, y } => println!("On the y-axis at {y}"),
            Point { x, y } => println!("On neither axis: x = {x}, y = {y}"),
        }
    }

    {
        let msg = Message::ChangeColor(0, 160, 255);

        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {} and in the y direction {}", x, y);
            }
            Message::Write(text) => {
                println!("Text message: {}", text);
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b);
            }
        }
    }

    {
        let msg = NewMessage::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            NewMessage::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change the color to red {}, green {}, and blue {}", r, g, b);
            }
            NewMessage::ChangeColor(Color::Hsv(h, s, v)) => {
                println!(
                    "Change the color to hue {}, saturation {}, and value {}",
                    h, s, v
                );
            }
            _ => (),
        }
    }

    {
        let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });
        println!("The person is {feet} feet and {inches} inches tall, at coordinates ({x}, {y})");
    }

    {
        foo(3, 4);
    }

    {
        let mut setting_value = Some(5);
        let new_setting_value = Some(10);

        match (setting_value, new_setting_value) {
            (Some(_), Some(_)) => {
                println!("Can't overwrite an existing configuration.");
            }
            _ => {
                setting_value = new_setting_value;
            }
        }

        println!("setting is {:?}", setting_value);
    }

    {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, _, third, _, fifth) => {
                println!("Some numbers: {first}, {third}, {fifth}");
            }
        }
    }

    {
        let _x = 5;
        let y = 10;
        let s = Some(String::from("Hello"));

        if let Some(_s) = s {
            println!("found a string");
        }

        // println!("{:?}", s); // the value in s has been moved to _s

        let s = Some(String::from("Hello"));

        if let Some(_) = s {
            println!("found a string");
        }

        println!("{:?}", s);
    }

    {
        let origin = Point3D { x: 0, y: 0, z: 0 };

        match origin {
            Point3D { x, .. } => println!("x coordinate is {x}"),
        }
    }

    {
        let numbers = (2, 4, 8, 16, 32);

        match numbers {
            (first, .., last) => {
                println!("Some numbers: {first}, {last}");
            }
        }
    }

    {
        let num = Some(4);

        match num {
            Some(x) if x & 2 == 0 => println!("The number {x} is even"),
            Some(x) => println!("The number {x} is odd"),
            None => (),
        }
    }

    {
        let x = Some(5);
        let y = 10;

        match x {
            Some(50) => println!("Got 50"),
            Some(n) if n == y => println!("Matched, n = {n}"),
            _ => println!("Default case, x = {x:?}"),
        }

        println!("at the end: x = {x:?}, y = {y}");
    }

    {
        let x = 4;
        let y = false;

        match x {
            4 | 5 | 6 if y => println!("yes"), // (4 | 5 | 6) if y =>
            _ => println!("no"),
        }
    }

    {
        let msg = SimpleMessage::Hello { id: 5 };

        match msg {
            SimpleMessage::Hello { id: id_var @ 3..=7 } => {
                println!("Found an id in range: {}", id_var)
            }
            SimpleMessage::Hello { id: 10..=12 } => println!("Found an id in another range"),
            SimpleMessage::Hello { id } => println!("Found some other id: {}", id),
        }
    }
}

struct Point {
    x: i32,
    y: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

enum NewMessage {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

enum SimpleMessage {
    Hello { id: i32 },
}
