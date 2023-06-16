fn main() {
    println!("Hello, world!");

    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let userrrr = User {
        active: true,
        username: String::from("String"),
        email: String::from("@gmail.com"),
        sign_in_count: 1,
    };

    println!("user value is : {0}" ,{userrrr.username});
}
