fn main() {
    {
        let x: Option<i32> = None;
        let y = match x {
            Some(i) => Some(i + 1),
            None => None,
        };
        println!("y is {:?}", y);
    }

    {
        let favorite_color: Option<&str> = None;
        let is_tuesday = false;
        let age: Result<u8, _> = "34".parse();

        if let Some(color) = favorite_color {
            println!("Using your favorite color, {}, as the background", color);
        } else if is_tuesday {
            println!("Tuesday is green day!");
        } else if let Ok(age) = age {
            if age > 30 {
                println!("Using purple as the background color");
            } else {
                println!("Using orange as the background color");
            }
        } else {
            println!("Using blue as the background color");
        }
    }

    {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            for val in [1, 2, 3] {
                tx.send(val).unwrap();
            }
        });

        while let Ok(value) = rx.recv() {
            println!("{}", value);
        }
    }

    {
        let v = vec!['a', 'b', 'c'];

        for (index, value) in v.iter().enumerate() {
            println!("{} is at index {}", value, index);
        }
    }

    {
        let (x, y, z) = (1, 2, 3);
        println!("x: {}, y: {}, z: {}", x, y, z);
    }

    {
        let point = (3, 5);
        print_coordinates(&point);
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
