// User struct definition
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// Function to create instance of User struct
fn build_user(email: String, username: String) -> User {
    User {
        email, // Field init shorthand for email: email
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Using tuple structs without named fields to create different types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like structs without any fields
struct AlwaysEqual;

fn main() {
    // User struct instantiation
    let mut user1 = User {
        email: String::from("user1@mail.com"),
        username: String::from("username1"),
        active: true,
        sign_in_count: 1,
    };

    // Changing the value of a field needs whole struct to be mutable
    user1.email = String::from("user1_new@mail.com");
    println!("User 1's active status: {}", user1.active);
    println!("User 1's username: {}", user1.username);
    println!("User 1's email: {}", user1.email);
    println!("User 1's sign_in_count: {}", user1.sign_in_count);

    // Using function to create instance of User struct
    let user2 = build_user(String::from("user2@mail.com"), String::from("username2"));
    println!("User 2's active status: {}", user2.active);
    println!("User 2's username: {}", user2.username);
    println!("User 2's email: {}", user2.email);
    println!("User 2's sign_in_count: {}", user2.sign_in_count);

    // Creating instances from other instances
    let user3 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("user3@mail.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("User 3's active status: {}", user3.active);
    println!("User 3's username: {}", user3.username);
    println!("User 3's email: {}", user3.email);
    println!("User 3's sign_in_count: {}", user3.sign_in_count);

    // Creating instances from other instances with struct update syntax
    let user4 = User {
        email: String::from("user3@mail.com"),
        ..user2
    };

    println!("User 4's active status: {}", user4.active);
    println!("User 4's username: {}", user4.username);
    println!("User 4's email: {}", user4.email);
    println!("User 4's sign_in_count: {}", user4.sign_in_count);

    // black and origin values are of different types
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!(
        "First value in Black: {} and in Origin: {}",
        black.0, origin.0
    );

    let _subject = AlwaysEqual;
    // Error[E0277]: `AlwaysEqual` doesn't implement `std::fmt::Display`
    // println!("{}", subject);
}
