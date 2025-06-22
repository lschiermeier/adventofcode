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
    let outvec: Vec<Vec<String>> = vec![];
    match lines {
        Err(err) => Err(err),
        Ok(_) => {
            Ok(outvec)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::read_lines;

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }

    #[test]
    fn test_read_table() {
        match read_lines("testinput/read_table.txt") {
            Err(_) => assert!(false),
            Ok(table) => {
                assert!(true);
            }
        }
    }
}

