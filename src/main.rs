use std::fmt::Display;

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

impl Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Username: {}, Email: {}", self.username, self.email)
    }
}

fn main() {
    println!("{}", build_user(String::from("someusername123"), String::from("someone@example.com")));
   

}

fn build_user(email: String, username: String) -> User{
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}