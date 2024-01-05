use std::collections::HashMap;

use crate::Tile::{Empty, Wall};

use rand::Rng;


#[derive(Clone, PartialEq)]
pub enum Tile {
    Wall,
    Empty,
    End
}


impl Tile {
    fn to_string(&self) -> &'static str {
        match self {
            Tile::Wall => "█",
            Tile::Empty => " ",
            Tile::End => "X"
        }
    }
}

fn render_map(map: &Vec<Vec<Tile>>, tick_count: u128) {
    print!("\x1B[2J\x1B[1;1H");

    println!("==================== {} ====================", tick_count);
    for row in map {
        println!("{}", row.into_iter().map(|tile| tile.to_string()).fold(String::from(""), |acc, cur| acc + cur));
    }
    println!("====================");
}


mod maze {
    use crate::Tile::{Empty, Wall, End};

    use crate::Tile;

    use rand::Rng;

    use rand::seq::SliceRandom;

    #[derive(Clone)]
    pub struct Point {
        pub r: i32,
        pub c: i32
    }
    const X: [i32; 4] = [0, 1, 0, -1];
    const Y: [i32; 4] = [1, 0, -1, 0];

    pub struct MazeMaker {
        pub map: Vec<Vec<Tile>>,
        point: Point
    }

    impl MazeMaker {
        pub fn new(mut map: Vec<Vec<Tile>>) -> MazeMaker {
            let start_point = Point {r: 1, c: 1};
            map[start_point.r as usize][start_point.c as usize] = Empty;
            MazeMaker {
                map,
                point: start_point
            }
        }

        pub fn make_maze(self: &mut MazeMaker) {
            let mut dirs: Vec<usize> = (0..4).collect();
            dirs.shuffle(&mut rand::thread_rng());
            let point = self.point.clone();
            for dir in dirs {
                let nr = point.r + X[dir];
                let nc = point.c + Y[dir];
                if nr <= 0 || nr >= (self.map.len() - 1) as i32 || nc <= 0 || nc >= (self.map[0].len() - 1) as i32 {
                    continue;
                }
                if self.map[nr as usize][nc as usize] == Wall && self.is_point_surround_by_3wall(Point{r: nr, c: nc}) {
                    self.map[nr as usize][nc as usize] = Empty;
                    self.point = Point {r: nr, c: nc};
                    self.make_maze();
                }
            }
        }

        pub fn is_point_surround_by_3wall(&self, point: Point) -> bool {
            let mut wall_count = 0;
            for dir in 0..4 {
                let nr = point.r + X[dir];
                let nc = point.c + Y[dir]; 
                if self.map[nr as usize][nc as usize] == Wall {
                    wall_count += 1;
                }
            }
            return wall_count >= 3;
        }
    }

}


fn make_maze(map: &mut Vec<Vec<Tile>>, point: maze::Point) {

}

fn main() {
    
    let mut tick_count: u128 = 0;
    const TICKS_PER_SECOND: u32 = 60;
    const SKIP_TICKS: u32 = 1000 / TICKS_PER_SECOND;
    
    let mut next_tick = std::time::Instant::now();
    let N = 20;
    let M = 20;

    // todo: init map by size
    let next_map: Vec<Vec<Tile>> = vec![vec![Wall; N + 2]; M + 2];
    let mut maze_maker = maze::MazeMaker::new(next_map);
    loop {
        tick_count+=1;
        // caclculate tick count
        while std::time::Instant::now() < next_tick {
            std::thread::sleep(std::time::Duration::from_millis(1));
        }
        next_tick = std::time::Instant::now() + std::time::Duration::from_millis(SKIP_TICKS as u64);

        // todo: find map algorithm implement
        // let mut next_map = vec![vec![Wall, Wall, Wall],vec![Wall, Wall, Wall],vec![Wall, Wall, Wall]];
        // let mut next_map: Vec<Vec<Tile>> = [row; 20].into();

        maze_maker.make_maze();


        render_map(&maze_maker.map, tick_count);
    }
}