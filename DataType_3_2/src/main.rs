fn main() {
    // MARK: TypeAnnotaion이 필요함 왜? 추론을 안하지?
    let guess = "42".parse().expect("Not a number!");
    println!("the value is {guess}");
}

// Rust의 정수형은 (8~128 and arch) 와 i u의 조합으로 12가지 조합이 가능함
// in은 -2^(n-1) ~ (2^n) - 1 u는 0 ~ 2^n - 1
// 수 리터럴은 98_222, 0x, 0o, 0b, b'A' <- u8

// Integer Overflow 처리가 되어있어서 debug모드에선 런타임 크래시가 발생할 수 있지만 릴리즈 모드는 방어해줌
// 명시적으로 처리하려면 wrapping_ or checked_ overflowing_ saturating_ 등을 이용

// floating pointsms f32 or f624를 쓰고 대부분의 cpu 환경에선 f62기본이고 성능차이가 별로 안남 - double precision
// 

// 암시적 형변환은 없음
// fn test() {
//     let guess = 5 / 2.3;
// }

// Rust의 char type은 4바이트이고  Unicode 표현이 가능함

// Compound Type에는 Tuple과 Array가 있음
// tuple은 destructuring 문법이 가능함
// let (x, y, z) = tup;
// 특별히 네이밍은 안되고 index로 가리켜야하는듯?

// fn test() {
    // let tup = (1, 1, 1, 1);
    // tup.5;
// }

// Array는 두가지 방법으로 선언가능함
// let a: [i32; 5] = [1, 2, 3, 4, 5];
// let a = [3; 5];
// fn test() {
//     let a = [3; 5];
        // 이렇게 선언한다고 컴파일타임에 이걸 체크해주지는 않는듯
//     a[7];
// }
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
// 대신 런타임 오류 발생