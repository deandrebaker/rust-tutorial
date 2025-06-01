struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        // The whole struct is either mutable or immutable. Specific fields
        // can't be mutable while others are immutable.
        active: true,
        username: "deandrebaker".to_string(),
        email: "deandrebaker@gmail.com".to_string(),
        sign_in_count: 1,
    };

    user1.active = false;

    println!(
        "username: {}, email: {}, active: {}, sign_in_count: {}",
        user1.username, user1.email, user1.active, user1.sign_in_count
    );

    let user2 = build_user("bluebeast@gmail.com".to_string(), "bluebeast".to_string()); // Takes
    // ownership of result of build_user

    println!(
        "username: {}, email: {}, active: {}, sign_in_count: {}",
        user2.username, user2.email, user2.active, user2.sign_in_count
    );

    let deactivated_user2 = User {
        active: false,
        ..user2 // Takes ownership of user2 because the email and username fields are moved not
                // copied
    };

    println!(
        "username: {}, email: {}, active: {}, sign_in_count: {}",
        deactivated_user2.username,
        deactivated_user2.email,
        deactivated_user2.active,
        deactivated_user2.sign_in_count
    );

    let user3 = User {
        email: "deandrebaker2@gmail.com".to_string(),
        ..user1 // active and sign_in_count are copied because they are of type u64 and bool.
                // username is moved because it is of type String.
    };

    println! {"user1 email: {}", user1.email}; // email is still accessible since it was not moved
    // to user3

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let Color(r, g, b) = black;
    let Point(x, y, z) = origin;
    println!(
        "Color: ({}, {}, {}), Point: ({}, {}, {})",
        black.0, black.1, black.2, origin.0, origin.1, origin.2
    );
    println!("Color: ({}, {}, {})", r, g, b);
    println!("Point: ({}, {})", x, y);

    let subject = AlwaysEqual;
}
