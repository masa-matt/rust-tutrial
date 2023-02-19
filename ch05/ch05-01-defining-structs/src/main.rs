fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // error[E0594]: cannot assign to `user1.email`, as `user1` is not declared as mutable
    // user1.email = String::from("anotheremail@example.com");

    let user2 = build_user(String::from("someone@example.com"), String::from("someone"));

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    tuple_struct();
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        // email: email,
        // username: username,
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn tuple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

struct UserWithReferences {
    // error[E0106]: missing lifetime specifier
    // username: &str,
    // email: &str,
    sign_in_count: u64,
    active: bool,
}

fn struct_with_references() {
    let user = UserWithReferences {
        // email: "someone@example.com",
        // username: "someusername123",
        active: true,
        sign_in_count: 1,
    };
}
