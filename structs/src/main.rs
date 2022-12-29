#![allow(unused_variables, dead_code)]

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: i64,
}

struct AlwaysEqual;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let mut user2 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user2.email = String::from("example@example.com");

    // println!("{:#?}", user1);

    let user3 = build_user("username3", "user3@example.com");

    let user4 = User {
        username: String::from("username4"),
        ..user3
    };

    // println!("{} {} {} {}", user1, user2, user3, user4);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    let subject = AlwaysEqual;
}

fn build_user(username: &str, email: &str) -> User {
    User {
        email: email.to_string(),
        username: username.to_string(),
        active: true,
        sign_in_count: 1,
    }
}
