// Cargo는 Rust의 package manager의 기본 <- 패키지 매니저를 헬로우 월드 다음에 소개하는 책은 처음봄
// cargo init or cargo new directory 를 통해 main.rs를 쉽게 만들 수 있다. 하지만 1_3_...으로 디렉토리를 생성했더니 패키지 생성이 되지않는 문제가 있었음
// 그 이후 cargo build를 이용
// $ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows
// or cargo run을 이용하면 실행파일이 바로 실행된다. 위의 빌드도 수행됨
// cargo check를 치면 쉽게 컴파일 체크를 할 수 있음
// cargo build --release 를 치면 릴리즈 가능
// cargo를 이용하면 rustc를 이용해서 컴파일하고 실행하는 것 이상으로 복잡한 구조를 쉽게 대응할 수 있음. 처음에 왜 패키지 매니저부터 설명하는지에 대한 의문이 해소됨
