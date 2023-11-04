fn main() {
    // 소유권 파트에서 함수의 매개변수로 힙 변수를 넣게되면 소유권이 이동하는데, 이러면 매우 피곤함
    // 따라서 참조를 이용해서 잠시 빌려줄 수 있음
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    // 빌린 값을 수정하려면 &mut를 이용해야만 함
    let mut s = String::from("hello");

    change(&mut s);

    // 하지만 변경 가능한 참조는 동시에 두가지를 생성할 수 없음
    // 이러한 제약은 컴파일 타임에 데이터 경합을 방지할 수 있기 때문에 유용함
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

// dangle인 상황은 조심해야할 필요가 있음
// fn dangle() -> &String {
//     let s = String::from("hello");

//     &s
// }