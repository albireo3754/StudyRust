fn main() {
    println!("Hello, world!");
    mut_scope();
}
// 모든 프로그램은 메모리를 관리해야됨. 3가지 방식을 사용하는데,
// 1. 가비지 콜랙터
// 2. 프로그래머가 명시적으로 메모리 할당, 해제 반복
// 3. 컴파일러가 확인하는 규칙을 이용하여 소유권 시스템으로 관리된다. <- 러스트가 사용하는 방식이고 해당 방식에서 벗어나면 컴파일이 되지않기 때문에 소유권 때문에 프로그램이 런타임 에 속도가 느려지지 않는다.

// Rust같은 시스템 프로그래밍 언어는 스택 or 힙에 값이 있는지 없는지 여부가 중요하다. 스택은 LIFO 이기 때문에 고정된 알려진 크기가 필요하고, 컴파일 시 크기가 알 수 없다면 힙에 저장해야함
// 힙에 메모리를 할당하려면 힙에 충분히 큰 빈 덩어리 지점이 있고 해당 위치에 값을 밀어넣은 뒤 주소인 포인터를 반환해서 스택에 저장하고 있음
// 따라서 스택이 힙보단 데이터 엑세스 속도가 빠르다. 따라서 코드의 어느 부분이 힙에 데이터를 사용하고 있는지, 힙에 있는 중복 데이터의 양을 최소화 하고, 공간이 부족하지 않도록 힙에서 사용되지 않는 데이터를 제거하는것이
// 소유권이 하는 일

fn mut_scope() {
    let s = " hello";

    {
        let s = " hello";
    }

    // 위의 코드에선 s가 유효한 범위는 일반적인 프로그래밍 언어와 같다.

    // String을 사용한 이유는 예시 설명에 적합한 타입이라서임. 문자열 리터럴 자체는 불변이지만 스트링 객체는 계속해서 변경되어질 수 있기때문에 힙에 저장되어야만 함
    // 이를 쉽게 메모리를 요청하고, 해제되어야만 하는 타이밍에 메모리가 해제되어야함. 너무 빨리 메모리를 해제해버리면 그것도 버그 사유가 된다.
    // 따라서 C++에서 많이 사용하는 RAll 패턴을 이용해 러스트는 스코프가 닫히는 시점에 drop 함수를 실행한다.

    let s1 = String::from("hello");
    // 만약 이 표현식으로 인해 s2와 s1이 같은 힙 메모리 주소를 가리킨다고 가정하면, 아까 얘기했던 scope가 닫힐때 실행 되는 drop 함수로 인해 메모리를 2번 해제하려고 하게되고, 이는 Double Free Error로 알려진
    // 메모리 안전 버그 중 하나를 야기할 수 있다.
    // 이 문제를 해결하기 위해선 rust에서는 shallow copy라고 하는 방식이 아닌, 첫번째 변수를 무효화하는 move 방식을 이용한다. 이렇게 스코프에서 벗어나게되면 s2의 메모리만 drop함
    let s2 = s1;
    // println!("${s1}");
    // 필요하면 clone을 이용해 힙 데이터를 딥카피 할 수 있음
    let s3 = s2.clone();
    println!("${s2}");

    // 정수형과 같이 스택 데이터는 그냥 copy가능함 or Copy trait을 구현하면 됨
    let x = 5;
    let y = x;
    println!("{}, {}", x, y); 
    
    // 이렇게 해도 소유권이 함수로 이동한다.
    let s4 = takes_ownershilp(s3);
    // println!("{}", s3);
    println!("{}", s4);
}

fn takes_ownershilp(some_string: String) -> String {
    some_string
}