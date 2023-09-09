fn main() {
    // 소유권 파트에서 함수의 매개변수로 힙 변수를 넣게되면 소유권이 이동하는데, 이러면 매우 피곤함
    // 따라서 참조를 이용해서 잠시 빌려줄 수 있음
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 빌린 값을 수정하려면 &mut를 이용해야만 함
    let mut s = String::from("hello");

    change(&mut s);

    let mut s2 = String::from("hello");

    let r1 = &mut s2;
    let r2 = &mut s2;

    println!("{}, {}", r1, r2);
}

fn calculate_length(s: &String) -> usize {
    // 빌린 값은 수정할 수 없다.
    // s = String::from("test");
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}