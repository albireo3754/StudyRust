use std::io;
use rand::seq::IteratorRandom;

fn main() {
    println!("Baseball Game!");

    let secret_numbers = pick_secret_numbers();

    println!("=================={:?}==================", secret_numbers);
    loop {
        println!("Please input 4-digits number: ");
        let mut guess = String::new();
        let _ = io::stdin()
            .read_line(&mut guess);

        let guess: String = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("retry! error:1");
                continue;
            },
        };
        
        if guess.len() != 4 {
            println!("retry! error:2");
            continue;
        }

        let mut inputs: [u32; 4] = [0; 4];
        for (index, c) in guess.chars().enumerate() {
            let num: u32 = c.to_digit(10).unwrap_or(0);
            inputs[index] = num;
        }

        let result = calculate_score(inputs, secret_numbers);

        if result.strike == 4 {
            println!("You Win!");
            return;
        } else {
            println!("{:?}", result);
        }
    }
}

fn pick_secret_numbers() -> [u32; 4] {
    let mut rng = rand::thread_rng();
    let result = (1..=9).choose_multiple(&mut rng, 4);
    return [result[0], result[1], result[2], result[3]];
}

#[derive(PartialEq)]
#[derive(Debug)]
struct CommandResult {
    strike: i32,
    ball: i32,
}

fn calculate_score(target: [u32; 4], secret_numbers: [u32; 4]) -> CommandResult {
    let mut strike = 0;
    let mut ball = 0;
    for i in 0..4 {
        if target[i] == secret_numbers[i] {
            strike += 1;
        } else if secret_numbers.contains(&target[i]) {
            ball += 1;
        }
    }
    return CommandResult { strike, ball };
}


#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::{pick_secret_numbers, calculate_score, CommandResult};

    #[test]
    fn test_pick_secret_numbers() {
        for _ in (1..100) {
            let results = pick_secret_numbers();
            let sets : HashSet<u32>= HashSet::from_iter(results);
            assert_eq!(sets.len(), results.len());
            assert_eq!(results.iter().min().unwrap() > &0, true);
            assert_eq!(results.iter().max().unwrap() < &10, true);
        }
    }

    #[test]
    fn test_calculate_score() {
        assert_eq!(calculate_score([1, 2, 3, 4], [1, 2, 3, 4]), CommandResult { strike: 4, ball: 0 });
        assert_eq!(calculate_score([1, 2, 3, 4], [2, 3, 4, 1]), CommandResult { strike: 0, ball: 4 });
        assert_eq!(calculate_score([1, 2, 3, 4], [5, 6, 7, 8]), CommandResult { strike: 0, ball: 0 });
        assert_eq!(calculate_score([1, 2, 3, 4], [5, 1, 7, 8]), CommandResult { strike: 0, ball: 1 });
    }
}