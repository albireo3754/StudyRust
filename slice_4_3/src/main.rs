fn main() {
    // let s = String::from("hello world");

    // let hello = &s[0..5];
    // let world = &s[6..11];
    let mut s = String::from("hello world");
    
    // let word = first_word(&s);
    let word2 = develop_first_word(&s[0..6]);


    s.clear(); // error!

    println!("the first word is: {}", word2);
}

// slice를 이용할때도 s에 대한 불변참조가 계속해서 이어지기 때문에 가변참조가 불가능함
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn develop_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}