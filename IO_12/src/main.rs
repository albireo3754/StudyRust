// cargo run -- a1 a2
use std::env;
use std::fs;

// 이런식으로 코드를 짜면 메인에 관심사가 다 모여서 복잡도가 올라가고 유지보수하기 어렵다.
// 그리고 에러메시지가 충분한 정보를 전달하지 않음
// fn main() {
//     // 잘못된 유니코드가 있다면 크래시가 나니깐 args_os를 이용하면 더 좋음
//     let args: Vec<String> = env::args().collect();
//     println!("Hello, world!");
//     let file_path: String = "test.txt".to_string();
//         // --snip--
//     println!("In file {}", file_path);

//     let contents = fs::read_to_string(file_path)
//     .expect("Should have been able to read the file");

//     println!("With text:\n{contents}");
// }

// parse로직 분리

// fn main() {
//     let args: Vec<String> = env::args().collect();

//     let (query, file_path) = parse_config(&args);

//     // --snip--
// }

// fn parse_config(args: &[String]) -> (&str, &str) {
//     let query = &args[1];
//     let file_path = &args[2];

//     (query, file_path)
// }

// Config 파일로 변경
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).expect("msg");

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}

struct Config {
    query: String,
    file_path: String,
}

// fn parse_config(args: &[String]) -> Config {
//     // Clone을 이용하면 소유권을 더이상 신경쓰지 않아도되는데 Trade off 가 존재함
//     // 런타임 이점이 좀 떨어지지만 쿼리스트링은 매우 작은편이기 때문에 일단 이렇게 작업을 하고, 최초 작업시에는 효율성 보다는 더 깔끔한 코드를 짜는데 집중한다.
//     let query = args[1].clone();
//     let file_path = args[2].clone();

//     Config { query, file_path }
// }

// parse로직을 Config 생성자 내부로 책임을 이동

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}