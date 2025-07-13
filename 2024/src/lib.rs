use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
                let mut lvec: Vec<String> = vec![];
                if line.as_ref().expect("Line empty?").len() > 0 {
                    for split in line.unwrap().split_whitespace() {
                        lvec.push(split.to_owned());
                    }
                }
                outvec.push(lvec);
            }
            Ok(outvec)
        }
    }
}

pub fn gen_input_path(day_rs_name: &str, test_mode: bool) -> Option<&Path> {
    if let length = day_rs_name.len() {
        
    }
    match day_rs_name.to() {
         => None,
        _ => panic!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_table() {
        let ref_table:Vec<Vec<&str>> = 
        vec![
            vec!["1","2","3","45","777"],
            vec!["23"],
            vec!["214"],
            vec!["142"],
            vec![],
            vec!["2222", "44", "24"]
        ];
        match read_table("testinput/read_table.txt") {
            Err(_) => assert!(false),
            Ok(table) => {
                assert_eq!(ref_table, table);
            }
        }
    }
    
    #[test]
    fn test_gen_input_path() {
        let path = gen_input_path("day10.rs", true);
    }
}

