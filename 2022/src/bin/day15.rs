use aoc_2022::*;
use regex::Regex;

fn main() {
    let lines = read_lines("inputs/day15_test.txt").expect("File Reading failed.");
    let reg = Regex::new(
        r"Sensor at x=(?<sx>\d+), y=(?<sy>\d+): closest beacon is at x=(?<bx>[-\d]+), y=(?<by>[-\d]+)^$",
    ).expect("Regex didn't compile");
    let mut sensors: Vec<Sensor> = Vec::new();
    let mut beacons: Vec<Position> = Vec::new();
    for line in lines.map(|x| x.unwrap()) {
        let caps = reg.captures(line.as_str()).expect("regex failed");
        let sensor_x: i64 = caps.name("sx").unwrap().as_str().parse().expect("parse failed");
        let sensor_y: i64 = caps.name("sy").unwrap().as_str().parse().expect("parse failed");
        let beacon_x: i64 = caps.name("bx").unwrap().as_str().parse().expect("parse failed");
        let beacon_y: i64 = caps.name("by").unwrap().as_str().parse().expect("parse failed");

        let beacon = Position{x: sensor_x + beacon_x, y: sensor_y + beacon_y};
        let sensor = Sensor{pos: Position{x: sensor_x, y: sensor_y}, beacon: beacon};
        sensors.push(sensor);
        if !beacons.contains(&beacon) {
            beacons.push(beacon);
        }
    }
    
    let map: Vec<Vec<Status>> = Vec::new();
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Position{
    x: i64,
    y: i64
}

impl Position {
    fn dist(&self, other: &Position) -> i64 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Sensor {
    pos: Position,
    beacon: Position
}

enum Status {
    Beacon,
    NoBeacon,
    Unknown
}