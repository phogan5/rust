struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//Tuple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

//Unit structs
struct AlwaysEqual;

//A function that automatically passes the arguments as attribute values
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn main() {
    //How to instantiate a struct
    let mut user1 = User {
        active: true,
        username: String::from("username123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };
    
    //How to reference or change values in a struct
    user1.email = String::from("anotheremail@example.com");

    //Using dot notation to copy values from an existing instance
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    //Using struct update syntax to copy values from another struct
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    let black = Color(0,0,0);
    let origin = Point(0,0,0);

    let subject = AlwaysEqual;

}   


