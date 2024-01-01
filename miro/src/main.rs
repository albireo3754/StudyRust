use std::collections::HashMap;
use crate::Tile::{Empty, Wall};

enum Tile {
    Wall,
    Empty
}

impl Tile {
    fn to_string(self) -> &'static str {
        match self {
            Tile::Wall => "█",
            Tile::Empty => " "
        }
    }
}

fn render_map(map: Vec<Vec<Tile>>, tick_count: u128) {
    print!("\x1B[2J\x1B[1;1H");

    println!("==================== {} ====================", tick_count);
    for row in map {
        println!("{}", row.into_iter().map(|tile| tile.to_string()).fold(String::from(""), |acc, cur| acc + cur));
    }
    println!("====================");
}

fn main() {
    
    let mut tick_count: u128 = 0;
    const TICKS_PER_SECOND: u32 = 60;
    const SKIP_TICKS: u32 = 1000 / TICKS_PER_SECOND;
    
    let mut next_tick = std::time::Instant::now();

    // todo: init map by size



    loop {
        tick_count+=1;
        // caclculate tick count
        while std::time::Instant::now() < next_tick {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        next_tick = std::time::Instant::now() + std::time::Duration::from_millis(SKIP_TICKS as u64);

        // todo: find map algorithm implement

        let mut next_map = vec![vec![Wall, Wall, Wall],vec![Wall, Wall, Wall],vec![Wall, Wall, Wall]];

        let i = tick_count % 9 / 3;
        let j = tick_count % 9 % 3;

        next_map[i as usize][j as usize] = Empty;

        render_map(next_map, tick_count);
    }
}