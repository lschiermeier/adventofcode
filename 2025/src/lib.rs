use core::{fmt, panic};
use itertools::Itertools;
use log::warn;
use regex::{self, Regex};
use std::fmt::Display;
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

pub fn read_table<P>(filename: P, delim: char) -> io::Result<Vec<Vec<String>>>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename);
    let mut outvec: Vec<Vec<String>> = vec![];
    match lines {
        Err(err) => Err(err),
        Ok(lines) => {
            for line in lines {
                let line: String = line.unwrap();
                outvec.push(if line.len() > 0 {
                    line.split(delim)
                        .filter(|x| x.len() > 0)
                        .map(|x| x.to_owned())
                        .collect()
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

pub fn read_chars<P>(filename: P) -> io::Result<Vec<Vec<char>>>
where
    P: AsRef<Path>,
{
    let lines = read_lines(filename);
    let mut out_vec: Vec<Vec<char>> = vec![];
    match lines {
        Err(err) => Err(err),
        Ok(lines) => {
            for line in lines {
                out_vec.push(line.unwrap().chars().collect());
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
                .map(|y| y.parse::<T>().expect("Could not parse string table"))
                .collect()
        })
        .collect()
}

pub fn parse_chars(table: Vec<Vec<char>>) -> Vec<Vec<u32>> {
    table
        .iter()
        .map(|x| x.iter().filter_map(|y| y.to_digit(10)).collect())
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
        match read_table("../testinputs/read_table.txt", ' ') {
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

    match read_table("../testinputs/read_table.txt", ' ') {
        Err(_) => assert!(false),
        Ok(table) => {
            let nums_table = parse_table::<i64>(table);
            assert_eq!(nums_table, ref_table);
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point2d {
    pub x: i64,
    pub y: i64,
}

impl ops::Add<Point2d> for Point2d {
    type Output = Point2d;
    fn add(self, _rhs: Point2d) -> Point2d {
        Point2d {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
        }
    }
}

impl Point2d {
    pub fn is_in_bounds(self, outer_bound: Point2d) -> bool {
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
    pub fn as_offset(self: &Direction) -> Point2d {
        match self {
            North => Point2d { x: 0, y: -1 },
            Northwest => Point2d { x: -1, y: -1 },
            West => Point2d { x: -1, y: 0 },
            Southwest => Point2d { x: -1, y: 1 },
            South => Point2d { x: 0, y: 1 },
            Southeast => Point2d { x: 1, y: 1 },
            East => Point2d { x: 1, y: 0 },
            Northeast => Point2d { x: 1, y: -1 },
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
    pub outer_bound: Point2d,
}

impl<T> Map2D<T>
where
    T: Copy,
{
    pub fn new(block: Vec<Vec<T>>) -> Option<Self> {
        Some(Map2D {
            outer_bound: Point2d {
                x: block.get(0)?.len() as i64,
                y: block.len() as i64,
            },
            block: block,
        })
    }

    pub fn new_from<F>(elems: Vec<Vec<F>>) -> Option<Self>
    where
        F: Into<T>,
    {
        let block: Vec<Vec<T>> = elems
            .into_iter()
            .map(|row| row.into_iter().map(|v| v.into()).collect())
            .collect();
        Map2D::new(block)
    }

    pub fn from_elem_with_bound(outer_bound: Point2d, elem: T) -> Option<Self> {
        let mut block: Vec<Vec<T>> = vec![];
        for _ in 0..outer_bound.y {
            let mut row: Vec<T> = vec![];
            for _ in 0..outer_bound.x {
                row.push(elem);
            }
            block.push(row);
        }
        Map2D::new(block)
    }

    pub fn get(&self, point: Point2d) -> Option<T> {
        if point.is_in_bounds(self.outer_bound) {
            Some(self.block[point.y as usize][point.x as usize])
        } else {
            None
        }
    }

    pub fn set(&mut self, point: Point2d, value: T) -> Result<(), String> {
        if point.is_in_bounds(self.outer_bound) {
            self.block[point.y as usize][point.x as usize] = value;
            Ok(())
        } else {
            Err("Point out of bounds".into())
        }
    }

    pub fn iter_row(&self) -> impl Iterator<Item = &Vec<T>> {
        self.block.iter()
    }

    pub fn iter_elem(&self) -> impl Iterator<Item = (T, Point2d)> {
        self.block.iter().enumerate().flat_map(|(y, row)| {
            row.iter().enumerate().map(move |(x, &val)| {
                (
                    val,
                    Point2d {
                        x: x as i64,
                        y: y as i64,
                    },
                )
            })
        })
    }

    pub fn add_row(&mut self, row: Vec<T>) -> Result<(), String> {
        if row.len() != self.outer_bound.x as usize {
            Err("Tryed adding unfitting row.".to_owned())
        } else {
            self.block.push(row);
            Ok(())
        }
    }

    pub fn get_row(&self, index: usize) -> Option<&Vec<T>> {
        if index >= self.block.len() {
            None
        } else {
            Some(&self.block[index])
        }
    }
}

impl<T> std::fmt::Display for Map2D<T>
where
    T: Display,
    T: Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.block.iter() {
            for x in y.iter() {
                write!(f, "{}", x)?
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

// Transposes a 2D table, expects rectangular
pub fn transpose<T>(table: Vec<Vec<T>>) -> Option<Vec<Vec<T>>>
where
    T: Copy,
{
    if !table.iter().map(|c| c.len()).all_equal() {
        warn!("Table was not rectangular.");
        return None;
    }
    let mut row_iters = table.iter().map(|x| x.iter()).collect_vec();
    let mut out_table: Vec<Vec<T>> = vec![];
    for _ in 0..table.first()?.len() {
        let col = row_iters
            .iter_mut()
            .map(|x| x.next().copied().unwrap())
            .collect();
        out_table.push(col);
    }
    Some(out_table)
}

#[test]
pub fn test_transpose() {
    let in_table: Vec<Vec<i64>> = vec![vec![1, 2, 3, 45, 777], vec![23, 1, 2, 3, 4]];
    let ref_table: Vec<Vec<i64>> = vec![
        vec![1, 23],
        vec![2, 1],
        vec![3, 2],
        vec![45, 3],
        vec![777, 4],
    ];
    if let Some(out_table) = transpose(in_table) {
        assert_eq!(out_table, ref_table, "Transpose didn't match ref.");
    } else {
        assert!(false, "Transpose failed unexpectedly");
    };
    let wrong_table = vec![vec!["blubb"], vec!["blah", "blah"]];
    assert_eq!(
        None,
        transpose(wrong_table),
        "Transpose didn't produce None."
    );
}

pub fn eq_to<T: std::cmp::PartialEq + 'static>(x: T) -> impl Fn(&T) -> bool {
    move |y| y == &x
}

#[derive(Debug, Copy, Clone)]
pub struct Point3d {
    pub x: i64,
    pub y: i64,
    pub z: i64,
}

impl TryFrom<&Vec<i64>> for Point3d {
    type Error = String;
    fn try_from(value: &Vec<i64>) -> Result<Self, Self::Error> {
        if value.len() == 3 {
            if let Some((x, y, z)) = value.iter().next_tuple() {
                Ok(Point3d {
                    x: *x,
                    y: *y,
                    z: *z,
                })
            } else {
                Err("Point could not be built.".to_owned())
            }
        } else {
            Err("Vec of wrong length.".to_owned())
        }
    }
}

impl ops::Add<Point3d> for Point3d {
    type Output = Point3d;
    fn add(self, _rhs: Point3d) -> Point3d {
        Point3d {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

/// Calculate Square Distance
impl ops::BitXor<Point3d> for Point3d {
    type Output = i64;
    fn bitxor(self, rhs: Point3d) -> Self::Output {
        (self.x - rhs.x) * (self.x - rhs.x)
            + (self.y - rhs.y) * (self.y - rhs.y)
            + (self.z - rhs.z) * (self.z - rhs.z)
    }
}

impl Point3d {
    pub fn is_in_bounds(self, outer_bound: Point3d) -> bool {
        match (self.x, self.y, self.z) {
            (..=-1, _, _) => false,
            (_, ..=-1, _) => false,
            (_, _, ..=-1) => false,
            (x, y, z) => x < outer_bound.x && y < outer_bound.y && z < outer_bound.z,
        }
    }
}
