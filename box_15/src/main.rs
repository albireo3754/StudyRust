
// #[derive(Debug)]
// struct CustomSmartPointer
// {
//     data: String,
// }

// impl Drop for CustomSmartPointer 
// {
//     fn drop(&mut self) {
//         println!("Dropping HasDrop!");
//         self.data = String::from("");
//     }

// }

// fn main() {
//     let a = 'a';
//     let b = Box::new(5);
//     let b2 = Box::new('3');
//     let b4 = Box::new(a);
//     // But the reverse is not possible: immutable references will never coerce to mutable references
//     // &U -> &mut T로 변환은 불가능함 다른 변환은 가능하고
//     // *b4 = '6';

//     let c = *b;
//     let mut b3 = *b2;

//         let csp = CustomSmartPointer {data: String::from("HI")};
//         // std::mem::drop(csp); // 이거 아무것도 안하는데 그냥 소유권을 뺏어서 만료시키는 원리랑 같음

//         {
//         let k = csp;
//         }
        
//     b3 = 't';

//     println!("b2: {}, b3: {}", *b2, b3);
//     // let b = csp;
//     println!("Hello, world!");
// }

// // Drop trait은 결국 RAII 를 구현하기 위한 방법'

use std::{rc::{Weak, Rc}, cell::RefCell, borrow::BorrowMut, ops::Deref};

pub trait Messanger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messanger> {
    messanger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where T: Messanger
{
    pub fn new(messanger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messanger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;
        
        if percentage_of_max >= 1.0 {
            self.messanger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messanger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messanger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}
// fn main() {
//     let a = 5;
//     let b = a;

//     println!("b: {}", b);
//     println!("a: {}", a);
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::cell::{RefCell, Ref};

//     struct MockMessanger {
//         sent_message: RefCell<Vec<String>>,
//         sent_message2: Vec<String>,
//     }

//     impl MockMessanger {
//         fn new() -> MockMessanger
//         {
//             MockMessanger {
//                 sent_message: RefCell::new(Vec::new()),
//                 sent_message2: Vec::new()
//             }
//         }

//         fn send2(&mut self, message: &str)
//         {
//             self.sent_message2.push(String::from(message));
//         }
//     }

//     impl Messanger for MockMessanger {

//         fn send(&self, message: &str) {
//             {

//              let borrow1 = self.sent_message.borrow();
//             }
//             {

//                 // let borrow2 = self.sent_message.borrow_mut();
//                 let borrow3 = self.sent_message.borrow();
//                 let borrow4 = self.sent_message.borrow();
//             }
//             {

//                 self.sent_message.borrow_mut().push(String::from(message));
//             }
//         }
//     }

//     #[test]
//     fn test() {
//         let mut mockMessanger = MockMessanger::new();
//         let mut limitTracker = LimitTracker::new(&mockMessanger, 100);
//         limitTracker.set_value(80);
//         mockMessanger.send2("message");

//         assert_eq!(mockMessanger.sent_message.borrow().len(), 1);

//     }
// }

// 메모리릭

// ? 
// use crate::List::{Cons, Nil};
// use std::cell::RefCell;
// use std::rc::Rc;

// #[derive(Debug)]
// enum List {
//     Cons(i32, RefCell<Rc<List>>),
//     Nil
// }

// impl List {
//     fn tail(&self) -> Option<&RefCell<Rc<List>>> {
//         match self {
//             List::Cons(_, list) => Some(list),
//             List::Nil => None,
//         }
//     }
// }

// fn main() {
//     let b = Nil;
// }

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>
}

struct Box {
    value: Rc<String>
}

impl Deref for Box {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn hello_no_ref(name: &str) {
    println!("Hello, {}!", name);
}



fn main() 
{
    let root = Rc::new(Node {
         value: 0, 
         parent: RefCell::new(Weak::new()), 
         children: RefCell::new(vec![]) 
        });

        println!("leaf parent = {:?}", root.parent.borrow().upgrade());

    let mut branch = Rc::new(Node {
         value: 0, 
         parent: RefCell::new(Weak::new()), 
         children: RefCell::new(vec![])
        });
    
    root.children.borrow_mut().push(Rc::clone(&branch));
    println!("child will {:?} 2", Rc::strong_count(&branch));
    *branch.parent.borrow_mut() = Rc::downgrade(&root);
    let t = branch.parent.borrow_mut();

    let box2 = Box {
        value: Rc::from(String::from("archon"))
    };

    hello((*(*box2)).as_ref());

    let x = 5;
    let y = &x;
    let t = x;

    assert_eq!(5, *y); // y는 x의 레퍼런스이므로 *y는 x의 값 5를 나타냅니다.
    assert_eq!(&5, y); // y는 x의 레퍼런스이므로 *y는 x의 값 5를 나타냅니다.

}