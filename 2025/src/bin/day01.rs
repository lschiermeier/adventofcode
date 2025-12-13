use aoc_2025::*;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    let test_mode;
    if args.len() > 1 && args[1] == "test" {
        test_mode = true;
    } else {
        test_mode = false;
    }
    let src_name = file!();
    let input_path = gen_input_path(src_name, test_mode);
    let lines = read_strings(input_path).expect("File could not be opened.");

    let (sum_p1, sum_p2) = solve(&lines);
    println!("{src_name} - Result 1: {sum_p1}");
    println!("{src_name} - Result 2: {sum_p2}");
}

fn solve(data: &Vec<String>) -> (i64,i64) {
    let mut dial: i64 = 50;
    let mut p1_count_at_zero= 0;
    let mut p2_count_at_zero= 0;
    for line in data {
        if line.len() < 2 {
            continue;
        }
        let (dir, size): (&str, &str) = line.split_at(1);
        let size_num = size.parse::<i64>().unwrap();
        match dir {
            "L" => dial -= size_num,
            "R" => dial += size_num,
            _ => panic!("Unknown direction"),
        }
        while dial < 0 {
            dial += 100;
            p2_count_at_zero += 1;
        } 
        while dial >= 100 {
            dial -= 100;
            p2_count_at_zero += 1;
        }
        if dial == 0 {
            p1_count_at_zero += 1;
        }
    }
    return (p1_count_at_zero, p2_count_at_zero);
}
