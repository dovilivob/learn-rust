struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user_1: User = User {
        email: String::from("david.iii.chiu@gmail.com"),
        username: String::from("David Chiu"),
        sign_in_count: 1,
        active: true,
    };

    let name: String = user_1.username;
    user_1.username = String::from("Chiu Chieh Yi");
    let user_2 = build_user(String::from("wayne@2enter.art"), String::from("Wayne Chen"));
    let user_3 = User {
        email: String::from("jerry@2enter.art"),
        username: String::from("Jerry"),
        ..user_2
    };

    // Tuple Structs
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

