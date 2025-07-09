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
            let mut lvec: Vec<String> = vec![];
            for line in lines {
                if line.as_ref().expect("Line empty?").len() > 0 {
                    for split in line.unwrap().split_whitespace() {
                        lvec.push(split.to_owned());
                    }
                }
            }
            outvec.push(lvec);
            Ok(outvec)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }

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
                let mut ref_iter = ref_table.iter();
                for line in table {
                    let ref_line = ref_iter.next().unwrap().to_owned();
                }
            }
        }
    }
}

