#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}

impl User {
    // Instance method .
    fn say_hello_email(&self) -> String {
        String::from("Hello my email is: ") + &self.email
    }

    // Asociated function :: => static method
    fn say_hello() -> String {
        String::from("hello")
    }
}

#[derive(Debug)]
struct UserT(String, String, u64, bool);

fn main() {
    let user = User {
        username: String::from("juanitodread"),
        email: String::from("mymail@email.com"),
        sign_in_count: 1,
        active: true
    };

    // Struct as Tuple style
    let user2 = UserT(String::from("juanitodread"), String::from("other@mail.com"), 1, true);

    print_user(&user);
    print_user(&user);

    println!("{}", user.say_hello_email());
    println!("{}", user.say_hello_email());

    println!("{}", User::say_hello());

    println!("UserT: username={}, email={}, sign_in_count={}, active={}",
            user2.0,
            user2.1,
            user2.2,
            user2.3);
}

fn print_user(user: &User) -> () {
    println!("User: {:?}", user)
}
