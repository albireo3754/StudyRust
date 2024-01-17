use std::collections::{HashMap, BinaryHeap};
use crate::Tile::{Empty, Wall};

use rand::Rng;


#[derive(Clone, PartialEq)]
pub enum Tile {
    Wall,
    Empty,
    Scarab,
    End
}


impl Tile {
    fn to_string(&self) -> &'static str {
        match self {
            Tile::Wall => "█",
            Tile::Empty => " ",
            Tile::Scarab => "0",
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

fn render_maze(maze: &maze::Maze, tick_count: u128) {
    print!("\x1B[2J\x1B[1;1H");

    println!("==================== {} ====================", tick_count);
    for row in &maze.map {
        println!("{}", row.into_iter().map(|tile| tile.to_string()).fold(String::from(""), |acc, cur| acc + cur));
    }
    println!("====================");
}


mod maze {
    use std::cmp::Ordering;
    use std::collections::{BinaryHeap, HashMap};
    use crate::Tile::{Empty, Wall, End};

    use crate::Tile;

    use rand::Rng;

    use rand::seq::SliceRandom;

    #[derive(Clone, PartialEq, Eq)]
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
            let end_point = Point {r: (map.len() - 2) as i32, c: (map[0].len() - 2) as i32};
            map[end_point.r as usize][end_point.c as usize] = End;

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
                if self.map[nr as usize][nc as usize] == Wall || self.map[nr as usize][nc as usize] == End {
                    wall_count += 1;
                }
            }
            return wall_count >= 3;
        }
    }

    #[derive(PartialEq, Eq)]
    pub struct Node {
        weight: i32,
        point: Point
    }

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            match self.weight.partial_cmp(&other.weight) {
                Some(core::cmp::Ordering::Equal) => return Some(Ordering::Equal),
                ord => return ord,
            }
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.weight.cmp(&other.weight)
        }
    }

    fn make_path() -> Vec<Point> {
        todo!()
    }

    fn make_maze(map: &mut Vec<Vec<Tile>>, point: Point) -> Option<Vec<Point>> {
        let mut queue = BinaryHeap::new();
        let mut edgeMap: HashMap<Point, Point> = HashMap::new();
        queue.push(Node { weight: 0 , point });

        while !queue.is_empty() {
            let node = queue.pop().unwrap();
            
            if map[node.point.r as usize][node.point.c as usize] == End {
                return Option::Some(make_path());
            }
        }
        return Option::None;
    }

    pub struct Maze {
        pub map: Vec<Vec<Tile>>,
        pub weight: Vec<Vec<i32>>,
        pub point: Point,
        pub end_point: Point,
        queue: BinaryHeap<Node>,
    }

    impl Maze {
        pub fn new(map: Vec<Vec<Tile>>) -> Maze {
            let start_point = Point {r: 1, c: 1};
            let mut queue = BinaryHeap::new();
            let weight = vec![vec![i32::MIN; map[0].len()]; map.len()];
            queue.push(Node { weight: 0, point: start_point.clone()});
            let end_point = Point {r: (map.len() - 2) as i32, c: (map[0].len() - 2) as i32};

            Maze {
                map,
                weight,
                point: start_point,
                end_point: end_point,
                queue
            }
        }

        pub fn move_one_tick(&mut self) {
            if let Some(node) = self.queue.pop() {
                self.map[node.point.r as usize][node.point.c as usize] = Empty;
                let dirs: Vec<usize> = (0..4).collect();
                let current_weight = -node.weight - self.get_heuristic_distance(&node.point);
                for dir in dirs {
                    let nr = node.point.r + X[dir];
                    let nc = node.point.c + Y[dir];
                    if nr <= 0 || nr >= (self.map.len() - 1) as i32 || nc <= 0 || nc >= (self.map[0].len() - 1) as i32 {
                        continue;
                    }
                    if self.map[nr as usize][nc as usize] == Wall {
                        // 
                        continue;
                    }
                    let new_point = Point {r: nr, c: nc};
                    let new_weight = -(current_weight + 1 + self.get_heuristic_distance(&new_point));
                    if self.weight[nr as usize][nc as usize] < new_weight {
                        self.weight[nr as usize][nc as usize] = new_weight;
                        self.queue.push(Node { weight: new_weight, point: new_point});
                    }
                }
                let new_tick_point = self.get_current_tick();
                if (self.map[new_tick_point.r as usize][new_tick_point.c as usize] == Tile::End) {
                    self.queue.clear();
                }
                self.map[new_tick_point.r as usize][new_tick_point.c as usize] = Tile::Scarab;
            }
        }

        pub fn get_current_tick(&self) -> Point {
            let weight = -self.queue.peek().unwrap().weight;
            println!("weight: {:?}", weight);
            return self.queue.peek().unwrap().point.clone();
        }

        pub fn get_heuristic_distance(&self, point: &Point) -> i32 {
            // return 0;
            return (point.r - self.end_point.r).abs() + (point.c - self.end_point.c).abs();
        }
    }
}

fn main() {
    
    let mut tick_count: u128 = 0;
    const TICKS_PER_SECOND: u32 = 60;
    const SKIP_TICKS: u32 = 1000 / TICKS_PER_SECOND * 3;
    
    let mut next_tick = std::time::Instant::now();
    let N = 20;
    let M = 20;

    // todo: init map by size
    let next_map: Vec<Vec<Tile>> = vec![vec![Wall; N + 2]; M + 2];
    let mut maze_maker = maze::MazeMaker::new(next_map);
    maze_maker.make_maze();
    let mut maze = maze::Maze::new(maze_maker.map.clone());
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

        maze.move_one_tick();

        render_maze(&maze, tick_count);
    }
}