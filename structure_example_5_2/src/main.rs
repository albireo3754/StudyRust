#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: dbg!(30 * 2),
        height: 50,
    };

    println!(
        "The area of the rectangle is {:?} square pixels.",
        dbg!(rect1)
        // rect1
    );

    // println!(
    //     "The area of the rectangle is {:#?} square pixels.",
    //     rect1
    // );
}

fn area(rectangle: Rectangle) -> u32 {
    dbg!(rectangle.width * rectangle.height)
}