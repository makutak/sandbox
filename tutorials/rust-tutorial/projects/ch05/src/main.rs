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
}
