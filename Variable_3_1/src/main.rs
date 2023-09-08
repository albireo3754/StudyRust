// fn main() {
//     let x = 5;
//     println!("The value of x is: {x}");
//     x = 6;
//     println!("The value of x is: {x}");
// }
// 위의 코드는 컴파일에 실패한다. 오류: cannot assign twice to immutable variable
// 컴파일러 오류는 프로그램이 safe하지 않다는 것이지 너가 폐급이라는 것을 의미하지 않기 때문에 잘 읽자.
// immutable 하다는 의미를 컴파일 타임에 잡아낼 수 있다는 사실은 러스트에서 중요함. immutable은 버그를 만들어내는 주범이다.
// 그렇다고 모든 것을 immutable하게 만들 순 없다. mutable은 매우 유용하고 코드를 쓰기 쉽게 만든다.
// mutable하게 쓰려면 어떻게 할까?

// fn main() {
//     let mut x = 5;
//     println!("the value is {x}");
//     x = 6;
//     println!("the value is {x}");
// }
// Constants: name에 bound된 값 상수는 항상 불변임 Constatns를 선언하기 위해선 annotated되어야 한다는데 다음 챕터 Data type에서 나옴

fn main() {
    let x = 5;
    println!("the value is {x}");
    {
        let x = x + 1;
        println!("the value is {x}");
    }
    let x = x + 1;
    println!("the value is {x}");
    println!("the value is {x}");
}
// Shadowing: 일반적인 언어에서 scope를 통해서 같은 이름을 가진 변수를 선언하는 것은 흔한 일이지만,
// 이렇게 같은 스코프에서 같은 이름으로 선언하는것은 좀 신기하다. 그리고 mut 로 mutable하게 선언하는거랑 달리, 새로운 변수를 같은 이름으로 재활용하는 것이기 때문에 컴파일 타임에 체크하는 것
// 새로운 변수로 선언하기 때문에 타입이 다른것도 선언 가능 -> 개꿀
// let spaces = "   ";
// let spaces = spaces.len();
