use aoc_2022::*;

fn main() {
    let lines = read_lines("inputs/day12_test.txt").expect("couldn't open file");
    let mut map: Vec<Vec<u32>> = Vec::new();
    let mut start: (usize, usize);
    let mut goal: (usize, usize);
    for (y_index, line) in lines.map(|x| x.unwrap()).enumerate() {
        map.push(Vec::new());
        for (x_index, letter) in line.chars().enumerate() {
            let value = match letter {
                'S' => {
                    start = (y_index, x_index);
                    'a'
                },
                'E' => {
                    goal = (y_index, x_index);
                    'z'
                },
                value => value
            };
            map[y_index].push(value.to_digit(36).unwrap()-10);
        }
    }
}
