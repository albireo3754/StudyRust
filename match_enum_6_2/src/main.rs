enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    // in rust if not use ; <- this block will be return statement
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => {
            return 5;
        },
        Coin::Quarter => 25,
    };

    if let Coin::Dime = coin {
        ()
    };

    let a = 3;
    // other and _ difference?
    // match a {
    //     1 => println!("{}", 3),
    //     other => {
    //         println!("{}", 3);
    //         3
    //     },

    //     _ => {
    //         println!("{}", 3);
    //         3
    //     }
    // }

    // let b: Option<i32> = Some(3);
    // match b {
    //     Some(value) => println!("{}", value),
    //     _ => ()
    // }

    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    3
}

fn main() {
    println!("{}", value_in_cents(Coin::Dime));
    println!("Hello, world!");
}
