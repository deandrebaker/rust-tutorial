const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    // Mutability
    let mut x = 5;
    println!("the value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");

    // Constants
    println!("Three hours in seconds is: {THREE_HOURS_IN_SECONDS}");

    // Shadowing
    let x = 5;
    println!("the value of x the first time is: {x}");
    let x = x + 1;
    println!("the value of x the second time is: {x}");

    {
        let x = x * 2;
        println!("The value of x in the innter scope is: {x}");
    }

    println!("the final value of x is: {x}");
}
