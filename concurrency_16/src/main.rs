// # 16.1
// use std::thread;

// fn main() {
//     let v = vec![1, 2, 3];

//     // Closure는 기본적으로 환경을 capture할때 borrow를 함
//     // move 명시 필요
//     let handle = thread::spawn(move || {
//         println!("Here's a vector: {:?}", v);
//     });

//     handle.join().unwrap();
// }

// # 16.2
// use std::thread;
// use std::sync::mpsc;

// fn main() {
//     let (tx, rx) = mpsc::channel();

//     let threadCount = 5;

//     for i in 1..=threadCount {
//         let tx1 = tx.clone();
//         let is = i.to_string();
//         thread::spawn(move|| {
//             let val = String::from("hi with ") + is.as_str();
//             tx1.clone().send(val).unwrap();
//         });
//     }

//     let mut count = 0;
//     loop {
//         let received = rx.try_recv();
//         match received {
//             Ok(r) => {
//                 println!("Got: {}", r);
//                 count += 1;
//                 if (count == threadCount) {
//                     break;
//                 }
//             },
//             Err(e) => {
//                 println!("Got: {}", e);
//             }
//         }
//     }
// }

// # 16.3

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        // let counter = Arc::clone(&counter);
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            {
                // recursive lockㅇㄴ?
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}