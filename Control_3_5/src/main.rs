fn main() {
    // looop는 while true 랑 동치

    let mut counter = 0;

    'test: while counter < 20 {
        // 하지만 expression이 될수있는건 loop 뿐인듯        
       let result = while counter < 20 {
            counter += 1;
            println!("Hello, world!");
            // continue;
            if counter == 10 {
                // break문을 이용해서 값을 반환하기도 가능함
                break 20;
                break 'test;
            }
        };

        if result == 20 {
            break 'test;
        }
    }
    // let result = 
    // println!("result: {result}");
    
}
