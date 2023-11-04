#[cfg(test)]
mod tests {
    // 해당 use문을 이용해 super의 모듈을 상속받을 수 있음
    use adder;

    // 해당 매크로를 통해 테스트 러너에게 테스트 해야할 타겟을 알려줌
    #[test]
    fn it_works() {
        let result = add(2, 2);
        // 다른 언어에선 expected, real 로 나타내지만 rust에선 left, right로 표현됨
        // left, right값이 되기위해선 Partial_Equal과 Debug를 모두 채택해야함

        assert_eq!(result, 6);
    }

    #[test]
    fn another() {
        // panic을 테스트에 이용할 수 있지만 다른 방법도 있다.
        // panic!("Make this test fail");
        // assert! 매크로를 이용하면 내부 평가식이 false일 때만 panic을 일으킴
        // assert!(false);
    }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    // panic의 발생을 강제하는 테스트
    #[should_panic]
    fn panic_must() {
        panic!("z")
    }

    #[test]
    #[ignore]
    fn internal() {
        // ignore처리 해도 컴파일 과정은 포함
        // assert_eq!(4, internal_adder(2, 2));
    }

    #[test]
    // 여기선 Should Panic은 못쓰고 value.is_error() 같은걸로 체크해야하지만
    // 엄청 사용성이 좋겟다는 생각이 들긴 했음
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

// cargo test -- --test-threads=1 를 이용하면 기본적으론 parrerel하게
// 동작하는 러스트의 테스트를 시퀀스하게 만들 수 잇음
// cargo test -- --show-output 결과가 이뻐지긴 했는데 특별히?
// #[ignore] 로 ignore상태로 분류할 수 있고
// cargo test -- --ignored 주석처리보단 좋을듯
