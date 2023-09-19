fn main() {
    println!("Hello, world!");
    let mut user1 = User {
        active: true,
        username: String::from("hi"),
        email: String::from("some"),
        sign_in_count: 1 
    };
    user1.active = true;
    println!("{}",user1.email);
}

// Like js, this constructor allowed
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

// Rust 의 Struct simmilar to tuple, which not consider order of field
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

#[test]
fn test() {
    let mut user1 = User {
        active: true,
        username: String::from("hi"),
        email: String::from("some"),
        sign_in_count: 1 
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // regardless order fields
    let user3 = User {
        email: String::from("test"),
        username: String::from("test2"),
        ..user2
        
    };

    assert_eq!(user1.active, user2.active);
    assert_eq!(user1.active, user3.active);
}