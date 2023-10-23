use std::fmt::Display;

// 추상화가 반복됨
fn largest_i32(list: &[i32]) -> &i32 {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_char(list: &[char]) -> &char {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

// T를ㄹ 두군대 모두 써줘야함
// 다른 이름도 가능
impl<U> Point<U> {
    fn x(&self) -> &U {
        &self.x
    }
}

// 구체 타입이면 generic x
impl Point<f32> {
    fn y(&self) -> &f32 {
        &self.x
    }
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        // 비교를 위해선 T: std::cmp::PartialOrd를 써야한다.
        if item > largest {
            largest = item;
        }
    }

    largest
}

// mark: Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.
// Trait은 다른 특성을 갖는다는데 무슨 특성이 다를까?

// Trait부분에서 이게 이해를 좀 하기 어려운 부분?
// But we can’t implement external traits on external types. For example, we can’t implement the Display trait on Vec<T> within our aggregator crate, because Display and Vec<T> are both defined in the standard library and aren’t local to our aggregator crate. This restriction is part of a property called coherence, and more specifically the orphan rule, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.

// impl에는 Trait의 멤버가 아닌 함수를 구현할 수는 없음
impl Summary for Point<f32> {
    // fn summarize2(&self) -> String {
    //     String::from("(Read more...)")
    // }
}

pub fn notify(item: &(impl Summary + Display)) {
}

// Notify2와 notify3는 표기법만 다름
pub fn notify2<T: Summary + Display>(item: &T){}

pub fn notify3<T>(item: &T) where T: Summary + Display {}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// if 조건문으로 다형성 출력은 불가능함
// fn returns_summarizable(switch: bool) -> impl Summary {
//     if switch {
//         NewsArticle {
//             headline: String::from(
//                 "Penguins win the Stanley Cup Championship!",
//             ),
//             location: String::from("Pittsburgh, PA, USA"),
//             author: String::from("Iceburgh"),
//             content: String::from(
//                 "The Pittsburgh Penguins once again are the best \
//                  hockey team in the NHL.",
//             ),
//         }
//     } else {
//         Tweet {
//             username: String::from("horse_ebooks"),
//             content: String::from(
//                 "of course, as you probably already know, people",
//             ),
//             reply: false,
//             retweet: false,
//         }
//     }
// }


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// lifetime은 'a로 표현되어있어도 더 작은쪽을 따라간다는 얘시
// fn main() {
//     let string1 = String::from("long string is long");
//     let result;
//     {
//         let string2 = String::from("xyz");
//         result = longest(string1.as_str(), string2.as_str());
//     }
//     println!("The longest string is {}", result);
// }

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
    // fn summarize2(&self) -> String;
}

// 이런게 안되면 String Generator 역할을 하는 함수는 못만들어 내는건가?
// fn longest2<'a>(x: &str, y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str().clone()
// }

fn longest3(x: &str, y: &str) -> &str {
    let result = String::from("really long string");
    result.as_str()
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest_i32(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest_char(&char_list);
    println!("The largest char is {}", result);
}