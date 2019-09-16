#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user = User {
        username: String::from("test"),
        email: String::from("test@example.com"),
        sign_in_count: 1,
        active: true,
    };

    println!("user: {:?}", user);

    let mut user_2 = User {
        username: String::from("test_2"),
        email: String::from("test_2@example.com"),
        sign_in_count: 2,
        active: true,
    };
    assert_eq!(user_2.email, "test_2@example.com");

    user_2.email = String::from("updated@example.com");
    assert_eq!(user_2.email, "updated@example.com");

    let user_3: User = build_user(String::from("user_3@example.com"), String::from("user_3"));
    assert_eq!(user_3.username, "user_3");
    assert_eq!(user_3.email, "user_3@example.com");

    let new_user: User = User {
        username: String::from("new user"),
        email: String::from("new_user@example.com"),
        sign_in_count: user.sign_in_count,
        active: user.active,
    };
    assert_eq!(new_user.sign_in_count, user.sign_in_count);
    assert_eq!(new_user.active, user.active);
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}
