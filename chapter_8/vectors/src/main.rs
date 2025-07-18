fn main() {
    {
        let v: Vec<i32> = Vec::new();
    }

    {
        let v = vec![1, 2, 3];
    }

    {
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
    }

    {
        let v = vec![1, 2, 3, 4, 5];
        let third: &i32 = &v[2];
        // let hundred: &i32 = &v[100];
        println!("The third element is: {}", third);
        // println!("The hundredth element is: {}", hundred);

        let third: Option<&i32> = v.get(2);
        match third {
            Some(third) => println!("The third element is: {}", third),
            None => println!("There is no third element."),
        }
    }

    {
        let v = vec![1, 2, 3, 4, 5];
        // let does_not_exist = &v[100];
        let does_not_exist = v.get(100);
        match does_not_exist {
            Some(value) => println!("Value at index 100: {}", value),
            None => println!("No value at index 100, it is out of bounds."),
        }
    }

    {
        let mut v = vec![1, 2, 3, 4, 5];

        v.push(6);
        let first = &v[0];
        // v.push(7); // Uncommenting this line will cause a compile-time error

        println!("The first element is: {}", first);
    }

    {
        let v = vec![100, 32, 57];
        for i in &v {
            println!("{i}");
        }
    }

    {
        let mut v = vec![100, 32, 57];

        for i in &mut v {
            *i += 50;
        }

        for i in &v {
            println!("{i}");
        }
    }

    {
        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(String::from("blue")),
            SpreadsheetCell::Float(10.12),
        ];

        println!("Row contents:");
        for cell in &row {
            println!("{cell:?}");
        }
    }
}

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
