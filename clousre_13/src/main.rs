#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        fn  add_one_v1   (x: u32) -> u32 { x + 1 }
        let add_one_v2 = |x: u32| -> u32 { x + 1 };

        // 책에선 분명히 The add_one_v3 and add_one_v4 도 같은 행동을 한다는데 컴파일이 안됨 ㅎ;

        // let add_one_v3 = |x| { x + 1 };
        // let add_one_v4 = |x| x + 1;

        let expensive_closure = |num: u32| -> u32 {
            println!("calculating slowly...");
            num
        };
        // Closure를 넘겨 받는 함수
        user_preference.unwrap_or_else(|| self.most_stocked())

    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

use std::thread;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);

    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // mut 참조가 있기때문에 println!으로 소유권을 넘기는것은 불가능
    // println!("Before defining closure: {:?}", list);



    borrows_mutably();
    println!("After calling closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // example_closure가 사용되는 곳이 있어서 타입 추론이 가능하면 사용 가능함
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));

    let mut list2 = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut sort_operations = vec![];
    let mut value = String::from("by key called");
    sort_operations.push(value);
    // value.push('1');

    // 이건 왜 안되지? FnMut는 외부변수를 캡쳐해서 변경할 수 있다는 뜻 아닌가?
    list2.sort_by_key(|r| {
        sort_operations.push(String::new());
        // 캡쳐한 외부 변수를 변경하지 못한다는게 아니라 캡쳐한 외부변수를 함수 바깥으로 보내지 못한다는 뜻
        // sort_operations.push(value);
        r.width
    });
    println!("{:#?}", list2);
}