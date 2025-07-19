use core::{fmt, panic};
use regex::{self, Regex};
use std::fs::File;
use std::io::{self, BufRead};
use std::ops;
use std::path::Path;
use std::slice::Iter;
use std::str::FromStr;

use self::Direction::*;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn read_table<P>(filename: P) -> io::Result<Vec<Vec<String>>>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename);
    let mut outvec: Vec<Vec<String>> = vec![];
    match lines {
        Err(err) => Err(err),
        Ok(lines) => {
            for line in lines {
                let line = line.unwrap();
                outvec.push(if line.len() > 0 {
                    line.split_whitespace().map(|x| x.to_owned()).collect()
                } else {
                    vec![]
                })
            }
            Ok(outvec)
        }
    }
}

pub fn read_strings<P>(filename: P) -> io::Result<Vec<String>>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename);
    let mut out_vec: Vec<String> = vec![];
    match lines {
        Err(err) => Err(err),
        Ok(lines) => {
            for line in lines {
                out_vec.push(line.unwrap());
            }
            Ok(out_vec)
        }
    }
}

pub fn read_ascii<P>(filename: P) -> io::Result<Vec<Vec<u8>>>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename);
    let mut out_vec: Vec<Vec<u8>> = vec![];
    match lines {
        Err(err) => Err(err),
        Ok(lines) => {
            for line in lines {
                out_vec.push(line.unwrap().as_bytes().to_owned());
            }
            Ok(out_vec)
        }
    }
}

pub fn gen_input_path(day_rs_name: &str, test_mode: bool) -> String {
    let rx = Regex::new(r"day(\d\d)").unwrap();
    let my_match = rx.find(day_rs_name);
    let day = match my_match {
        None => panic!("gen_input_path failed to run regex: input was: {day_rs_name}"),
        Some(x) => x.as_str(),
    };
    if test_mode {
        ["inputs/", day, "_test.txt"].concat()
    } else {
        ["inputs/", day, ".txt"].concat()
    }
}

pub fn parse_table<T>(table: Vec<Vec<String>>) -> Vec<Vec<T>>
where
    T: FromStr,
    <T as FromStr>::Err: fmt::Debug,
{
    table
        .iter()
        .map(|x| {
            x.iter()
                .map(|y| y.parse::<T>().expect("Could not parse table"))
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_table() {
        let ref_table: Vec<Vec<&str>> = vec![
            vec!["1", "2", "3", "45", "777"],
            vec!["23"],
            vec!["214"],
            vec!["142"],
            vec![],
            vec!["2222", "44", "24"],
        ];
        match read_table("testinputs/read_table.txt") {
            Err(_) => assert!(false),
            Ok(table) => {
                assert_eq!(ref_table, table);
            }
        }
    }

    #[test]
    fn test_gen_input_path() {
        let path = gen_input_path("day10.rs", true);
        assert_eq!(path, "inputs/day10_test.txt");
        let path = gen_input_path("day10.rs", false);
        assert_eq!(path, "inputs/day10.txt");
    }
}

#[test]
fn test_parse_table() {
    let ref_table: Vec<Vec<i64>> = vec![
        vec![1, 2, 3, 45, 777],
        vec![23],
        vec![214],
        vec![142],
        vec![],
        vec![2222, 44, 24],
    ];

    match read_table("testinputs/read_table.txt") {
        Err(_) => assert!(false),
        Ok(table) => {
            let nums_table = parse_table::<i64>(table);
            assert_eq!(nums_table, ref_table);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Offset {
    pub x: i64,
    pub y: i64,
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
pub enum Direction {
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

#[derive(Debug, Clone)]
pub struct Map2D<T>
where
    T: Copy,
{
    block: Vec<Vec<T>>,
    outer_bound: Offset,
}

impl<T> Map2D<T>
where
    T: Copy,
{
    pub fn new(block: Vec<Vec<T>>) -> Self {
        Map2D {
            outer_bound: Offset {
                x: block.get(0).expect("Empty Sub Vec given to Map2D.").len() as i64,
                y: block.len() as i64,
            },
            block: block,
        }
    }

    pub fn get(self, point: Offset) -> Option<T> {
        if point.is_in_bounds(self.outer_bound) {
            Some(self.block[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    
}
