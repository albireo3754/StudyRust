
#[derive(Debug)]
struct CustomSmartPointer
{
    data: String,
}

impl Drop for CustomSmartPointer 
{
    fn drop(&mut self) {
        println!("Dropping HasDrop!");
        self.data = String::from("");
    }

}

fn main() {
    let a = 'a';
    let b = Box::new(5);
    let b2 = Box::new('3');
    let b4 = Box::new(a);
    // But the reverse is not possible: immutable references will never coerce to mutable references
    // &U -> &mut T로 변환은 불가능함 다른 변환은 가능하고
    // *b4 = '6';

    let c = *b;
    let mut b3 = *b2;

        let csp = CustomSmartPointer {data: String::from("HI")};
        // std::mem::drop(csp); // 이거 아무것도 안하는데 그냥 소유권을 뺏어서 만료시키는 원리랑 같음

        {
        let k = csp;
        }
        
    b3 = 't';

    println!("b2: {}, b3: {}", *b2, b3);
    // let b = csp;
    println!("Hello, world!");
}

// Drop trait은 결국 RAII 를 구현하기 위한 방법'
