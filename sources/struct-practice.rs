// an example which implements `struct`

fn main() {
    let user1 = build_user(String::from("mail@abdus.xyz"), String::from("Abdus"), 20);

    println!("{:#?}", user1);
}

#[derive(Debug)] 
struct User {
    name: String,
    age: u32,
    email: String,
    active: bool,
}

fn build_user(email: String, name: String, age: u32) -> User {
    User {
        name,
        email,
        age,
        active: true,
    }
}
