use aoc_2024::*;

fn main() {
    let src_name = file!();
    let input_path = gen_input_path(src_name, false);
    let table = read_table(input_path).expect("File could not be opened.");

    let num_table: Vec<Vec<i64>> = parse_table(table);
    let sum_p1 = num_table.iter().filter(|x| determine_safety(x)).count();

    println!("{src_name} - Result 1: {sum_p1}");
    // println!("{src_name} - Result 2: {}");
}

fn determine_safety(report: &Vec<i64>) -> bool {
    todo!();
}
