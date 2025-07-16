use aoc_2024::*;
// use log;

fn main() {
    let src_name = file!();
    let input_path = gen_input_path(src_name, false);
    let table = read_table(input_path).expect("File could not be opened.");

    let num_table: Vec<Vec<i64>> = parse_table(table);
    let sum_p1 = num_table.iter().filter(|x| determine_safety(x)).count();
    let sum_p2 = num_table
        .iter()
        .filter(|x| determine_dampened_safety(x))
        .count();

    println!("{src_name} - Result 1: {sum_p1}");
    println!("{src_name} - Result 2: {sum_p2}");
}

fn determine_safety(report: &Vec<i64>) -> bool {
    if report.is_sorted_by(|a, b| a < b && a + 3 >= *b) {
        // println!("{report:?}");
        // println!("report is sorted growing.");
        true
    } else if report.is_sorted_by(|a, b| a > b && *a <= b + 3) {
        // println!("{report:?}");
        // println!("report is sorted shrinking.");
        true
    } else {
        // println!("{report:?}");
        // println!("report is not or incorrectly sorted.");
        false
    }
}

fn determine_dampened_safety(report: &Vec<i64>) -> bool {
    if determine_safety(report) {
        return true;
    }
    for idx in 0..report.len() {
        let mut cut_report = report.clone();
        cut_report.remove(idx);
        if determine_safety(&cut_report) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    static REF_LIST_FAIL: [i64; 5] = [1, 2, 7, 8, 9];
    static REF_LIST_SUCCESS: [i64; 5] = [7, 6, 4, 2, 1];
    static REF_LIST_DEPENDS: [i64; 5] = [8, 6, 4, 4, 1];
    #[test]
    fn test_determine_safety() {
        assert!(determine_safety(&REF_LIST_SUCCESS.to_vec()));
        assert!(!determine_safety(&REF_LIST_FAIL.to_vec()));
        assert!(!determine_safety(&REF_LIST_DEPENDS.to_vec()));
    }

    #[test]
    fn test_determine_dampened_safety() {
        assert!(determine_dampened_safety(&REF_LIST_SUCCESS.to_vec()));
        assert!(!determine_dampened_safety(&REF_LIST_FAIL.to_vec()));
        assert!(determine_dampened_safety(&REF_LIST_DEPENDS.to_vec()));
    }
}
