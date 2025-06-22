fn main() {
    {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number in {number_list:?} is {largest}");
    }

    {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number in {number_list:?} is {largest}");

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number in {number_list:?} is {largest}");
    }

    {
        let number_list = vec![34, 50, 25, 100, 65];
        let largest = get_largest(&number_list);
        println!("The largest number in {number_list:?} is {largest}");

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        let largest = get_largest(&number_list);
        println!("The largest number in {number_list:?} is {largest}");
    }
}

fn get_largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
