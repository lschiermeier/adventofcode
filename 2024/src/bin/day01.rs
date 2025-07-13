use aoc_2024::*;

fn main() {
    let src_name = file!();
    let input_path = gen_input_path(src_name, false);
    let table = read_table(input_path).expect("File could not be opened.");
    let mut list_left: Vec<i64> = vec![];
    let mut list_right: Vec<i64> = vec![];
    for line in table {
        if line.len() < 2 {
            continue;
        }
        list_left.push(line[0].parse().unwrap());
        list_right.push(line[1].parse().unwrap());

        // println!("{list_left:?},\n {list_right:?}");
    }
    list_left.sort();
    list_right.sort();
    let mut sum_p1: i64 = 0;
    let mut sum_p2: i64 = 0;
    for (left, right) in list_left.iter().zip(list_right.iter()) {
        sum_p1 += (left - right).abs();

        sum_p2 += left * list_right.iter().filter(|x| *x == left).count() as i64;
    }
    println!("{src_name} - Result 1: {sum_p1}");
    println!("{src_name} - Result 2: {sum_p2}");
}
