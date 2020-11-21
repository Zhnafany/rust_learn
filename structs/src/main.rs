struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

//Turple struct define
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//error need lifetime to use ref as value
struct User {
    username: &str,
    email: &str,
    sign_in_count: u64,
    active: bool,
}


fn main() {
    //////////////////////////////
    //immutable instance of user struct
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    /////////////////////////////////////
    //mutable instance of user struct with changing value of struct field
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    //////////////////////////////////
    //get the values from user1 in user2 declaration
    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    //Turple struct instance create
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
    //variant of creating User instance without repeating same names
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
