fn main() {
    // MARK: TypeAnnotaion이 필요함 왜?
    let guess = "42".parse().expect("Not a number!");
    println!("the value is {guess}");
}
