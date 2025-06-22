fn main() {
    {
        let number_list = vec![34, 50, 25, 100, 65];
        let largest = get_largest_i32(&number_list);
        println!("The largest number in {number_list:?} is {largest}");

        let char_list = vec!['y', 'm', 'a', 'q'];
        let largest = get_largest_char(&char_list);
        println!("The largest number in {char_list:?} is {largest}");
    }

    {
        let number_list = vec![34, 50, 25, 100, 65];
        let largest = get_largest(&number_list);
        println!("The largest number in {number_list:?} is {largest}");

        let char_list = vec!['y', 'm', 'a', 'q'];
        let largest = get_largest(&char_list);
        println!("The largest number in {char_list:?} is {largest}");
    }

    {
        let integer = Point { x: 5, y: 10 };
        let float = Point { x: 1.0, y: 4.0 };
        // let mixed = Point { x: 1, y: 4.0 }; // This line won't work since x and y are different
        // types

        println!("Integer point: ({}, {})", integer.x, integer.y);
        println!("Float point: ({:.2}, {:.2})", float.x, float.y);
    }

    {
        let mixed = MixedPoint { x: 1, y: 4.0 };

        println!("Mixed point: ({}, {:.2})", mixed.x, mixed.y);
    }

    {
        let compass: Direction<&str, i32> = Direction::Compass("North");
        let angle: Direction<i32, f64> = Direction::Angle(45.0);

        match compass {
            Direction::Compass(dir) => println!("Compass direction: {}", dir),
            Direction::Angle(_) => (),
        }

        match angle {
            Direction::Compass(_) => (),
            Direction::Angle(deg) => println!("Angle direction: {:.2} degrees", deg),
        }
    }

    {
        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.x());
    }

    {
        let p = Point { x: 3.0, y: 4.0 };
        println!("Distance from origin: {:.2}", p.distance_from_origin());
    }

    {
        let p1 = MixedPoint { x: 5, y: 10.4 };
        let p2 = MixedPoint { x: "Hello", y: 'c' };
        let p3 = p1.mixup(p2);

        println!("Mixed point: ({}, {})", p3.x, p3.y);
    }
}

fn get_largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn get_largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn get_largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

#[derive(Debug)]
enum Direction<T, U> {
    Compass(T),
    Angle(U),
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> MixedPoint<T, U> {
    fn mixup<V, W>(self, other: MixedPoint<V, W>) -> MixedPoint<T, W> {
        MixedPoint {
            x: self.x,
            y: other.y,
        }
    }
}
