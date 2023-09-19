#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&mut self) -> u32 {
        self.width = 3;
        self.width * self.height
    }
    
    fn test(self) -> u32 {
        self.width
    }

    fn test2(&self) -> u32 {
        self.width
    }

    fn test3(&self, tz: &mut String) -> u32 {
        self.width
    }
}

impl Rectangle {
    // like static method
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // println!("{}", rect1.test());
    println!("{}", rect1.test2());
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("{}", Rectangle::square(3).area())
}