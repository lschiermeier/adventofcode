use self::Direction::*;
use aoc_2024::*;
use std::ops;
use std::slice::Iter;

#[derive(Debug, Copy, Clone)]
struct Offset {
    x: i64,
    y: i64,
}

impl ops::Add<Offset> for Offset {
    type Output = Offset;
    fn add(self, _rhs: Offset) -> Offset {
        Offset {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl Offset {
    pub fn is_in_bounds(self, outer_bound: Offset) -> bool {
        match (self.x, self.y) {
            (..=-1, _) => false,
            (_, ..=-1) => false,
            (x, y) => x < outer_bound.x && y < outer_bound.y,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    Northwest,
    West,
    Southwest,
    South,
    Southeast,
    East,
    Northeast,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [
            North, Northwest, West, Southwest, South, Southeast, East, Northeast,
        ];
        DIRECTIONS.iter()
    }
    pub fn as_offset(self: &Direction) -> Offset {
        match self {
            North => Offset { x: 0, y: -1 },
            Northwest => Offset { x: 1, y: -1 },
            West => Offset { x: 1, y: 0 },
            Southwest => Offset { x: 1, y: 1 },
            South => Offset { x: 0, y: 1 },
            Southeast => Offset { x: -1, y: 1 },
            East => Offset { x: -1, y: 0 },
            Northeast => Offset { x: -1, y: -1 },
        }
    }

    pub fn get_opposite(self: &Direction) -> Direction {
        match self {
            North => South,
            Northwest => Southeast,
            West => East,
            Southwest => Northeast,
            South => North,
            Southeast => Northwest,
            East => West,
            Northeast => Southwest,
        }
    }
}

fn main() {
    let src_name = file!();
    let input_path = gen_input_path(src_name, false);
    let block = read_ascii(&input_path).expect(&format!("File not found.{}", input_path));

    let sum_p1 = search_word_in_block("XMAS".as_bytes(), &block);
    println!("{src_name} - Result 1: {sum_p1}");

    let sum_p2 = search_cross_in_block("MAS".as_bytes(), &block);
    println!("{src_name} - Result 2: {sum_p2}");
}

fn search_cross_in_block(word: &[u8], block: &Vec<Vec<u8>>) -> i64 {
    let height = block.len();
    let width = block[0].len();
    let mut sum_occurences: i64 = 0;
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            sum_occurences += find_cross_occurences_at_position(
                word,
                block,
                Offset {
                    x: x as i64,
                    y: y as i64,
                },
            )
        }
    }
    sum_occurences
}

fn find_cross_occurences_at_position(word: &[u8], block: &Vec<Vec<u8>>, position: Offset) -> i64 {
    let height = block.len() as i64;
    let width = block[0].len() as i64;
    let outer_bound = Offset {
        x: width,
        y: height,
    };
    if !position.is_in_bounds(outer_bound) {
        panic!("Trying to count occurences out of bounds.")
    };
    if block[position.y as usize][position.x as usize] != word[1] {
        return 0;
    }
    let mut counter = 0;
    for dir in [Northeast, Southeast, Southwest, Northwest] {
        let dir_position = position + dir.as_offset();
        let opp_position = position + dir.get_opposite().as_offset();
        if !(dir_position.is_in_bounds(outer_bound) && opp_position.is_in_bounds(outer_bound)) {
            return 0;
        }
        // println!(
        //     "{:?}: {},{} - {},{}",
        //     position,
        //     block[dir_position.y as usize][dir_position.x as usize],
        //     word[0],
        //     block[opp_position.y as usize][opp_position.x as usize],
        //     word[2]
        // );
        if block[dir_position.y as usize][dir_position.x as usize] == word[0]
            && block[opp_position.y as usize][opp_position.x as usize] == word[2]
        {
            counter += 1;
        }
    }
    // println!("{counter}");
    if counter >= 2 { 1 } else { 0 }
}

fn search_word_in_block(word: &[u8], block: &Vec<Vec<u8>>) -> i64 {
    let height = block.len();
    let width = block[0].len();
    let mut sum_occurences: i64 = 0;
    for x in 0..width {
        for y in 0..height {
            sum_occurences += count_word_occurences_at_position(
                word,
                block,
                Offset {
                    x: x as i64,
                    y: y as i64,
                },
            )
        }
    }
    sum_occurences
}

fn count_word_occurences_at_position(word: &[u8], block: &Vec<Vec<u8>>, position: Offset) -> i64 {
    let height = block.len() as i64;
    let width = block[0].len() as i64;
    let outer_bound = Offset {
        x: width,
        y: height,
    };
    if !position.is_in_bounds(outer_bound) {
        panic!("Trying to count occurences out of bounds.")
    }
    let mut counter = 0;
    'dir: for dir in Direction::iterator() {
        let mut current_offset: Offset = Offset { x: 0, y: 0 };
        for letter in word {
            let current_position = position + current_offset;
            if !current_position.is_in_bounds(outer_bound) {
                continue 'dir;
            } else if *letter != block[current_position.y as usize][current_position.x as usize] {
                continue 'dir;
            }
            current_offset = current_offset + dir.as_offset();
        }
        counter += 1;
    }
    counter
}
