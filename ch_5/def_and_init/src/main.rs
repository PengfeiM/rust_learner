//! first, let's define a struct.
//! * something like tuple: has values with different types.
//! * something different from tuple: every value has a name.

/// User: a struct example.
/// wich fileds: active, username, email, sign_in_count.
struct User {
    /// valid?
    active: bool,
    /// name of a user.
    username: String,
    /// email of a user.
    email: String,
    /// store the times of user's signin.
    sign_in_count: u64,
}

/// print_user
/// print content of user.
fn print_user(user: &User) {
    println!(
        "User:\n* active: {0}\n* username: {1}\n* email: {2}\n* count: {3}\n",
        user.active, user.username, user.email, user.sign_in_count,
    );
}

/// build_user
/// create a user instance with given usernam and email.
/// Shorthanded!!!
fn build_user(username: String, email: String) -> User {
    // User {
    //     active: true,
    //     username: username,
    //     email: email,
    //     sign_in_count: 0,
    // }
    // don't you feel annoying by using same name for params as fields?
    // let's do a little Shorthand.
    User {
        active: true,
        username, // <=> username: username,
        email,    // <=> email:email,
        sign_in_count: 0,
    }
}

fn do_some_struct() {
    // Initialize
    // create a variable with type User(struct).
    let user1 = User {
        active: true,
        username: String::from("someuser1name"),
        email: String::from("someuser1@some.domain"),
        sign_in_count: 1,
    };
    // to access a field of a struct, use dot ".".
    println!(
        "User1:\n* active: {0}\n* username: {1}\n* email: {2}\n* count: {3}\n",
        user1.active, user1.username, user1.email, user1.sign_in_count,
    );

    // also, "." can be used to assign value to a field.
    let mut user2 = User {
        active: true,
        username: String::from("someuser2namewrong"),
        email: String::from("someuser2@some.domain"),
        sign_in_count: 2,
    }; // Noted: Rust doesn't allow us to make only certain fields as mutable.

    user2.username = String::from("someuser2name");
    println!(
        "User2:\n* active: {0}\n* username: {1}\n* email: {2}\n* count: {3}\n",
        user2.active, user2.username, user2.email, user2.sign_in_count,
    );

    // like other language, let's make a struct builder/newer.
    let user3 = build_user(
        String::from("someuser3name"),
        String::from("someuser3@some.domain"),
    );
    print_user(&user3);

    // sometimes, we wanna create a struct with some data from another existing one.
    // let user4 = User {
    //     active: user3.active,
    //     username: user3.username,
    //     email: String::from("someuser4@some.domain"),
    //     sign_in_count: user3.sign_in_count,
    // };
    // it seems that we meet the some annoying thing like building user.
    // here's a solution in Rust.
    let user4 = User {
        email: String::from("someuser4@some.domain"),
        ..user3 // just expand user3 in user4
    };
    print_user(&user4);
}

/*
* some tuple struct.
* tuple struct don't have name for fields, just types.
* it's useful when you want to name a tuple and make it different from another.
* */
/// Color
/// rgb color.
struct Color(u8, u8, u8);
/// Point
/// point in 3-dimension space.
struct Point(u8, u8, u8);

fn do_some_tuple_struct() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    // although Color and Point have same filed types, but they are different types.

    println!(
        "Color: {{red: {0}, green: {1}, blue: {2}}}\n",
        black.0, black.1, black.2
    );
    println!(
        "Point: {{red: {0}, green: {1}, blue: {2}}}\n",
        origin.0, origin.1, origin.2
    );
}

/// AlwaysEqual
/// a unit struct example
/// a unit struct is a struct without any field.
struct AlwaysEqual;

fn do_some_unit_struct() {
    let subject = AlwaysEqual;
    println!("{:p}", &subject);
}

fn main() {
    // do some struct
    do_some_struct();

    // different from other language, Rust support a unique type: "tuple struct".
    do_some_tuple_struct();

    // do some unit struct
    do_some_unit_struct();
}
