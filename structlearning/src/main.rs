


struct Book
{
    title: String,
    author: String,
    uuid: i64,
    pages: i32,
}

struct User
{
    name: String,
    active: bool,
    email: String,
    sign_in_count: u64,
}


fn main() {

    //to allow us to change the values of this struct we must make this mutable
    let mut user1: User = User{
        active: true,
        name: String::from("Jeff"),
        email: String::from("fake.email@gmail.com"),
        sign_in_count: 100,
    };

    println!("Current email = {}", user1.email);
    //we can edit this email becuase the struct initialization is mutable
    user1.email = String::from("new.fake.email@gmail.com");

    println!("Current email = {}", user1.email);


    let user2: User = User{
        email: String::from("Another.fake.email@gmail.com"),
        ..user1 //this notation right here will copy all other fields from the first struct to the new one.
    };


    println!("New user is {}, old user name is {}", user2.name, user2.name);
}
