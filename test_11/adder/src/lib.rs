// $ cargo new adder --lib
//      Created library `adder` project
// $ cd adder
// 테스트 실행

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

struct Test {
    test: u32
}