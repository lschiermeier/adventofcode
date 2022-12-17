use aoc_2022::*;
use regex::Regex;

// No Space Left On Device
fn main() {
    let lines = read_lines("inputs/day07.txt").expect("File couldn't be read.");

    // let mut current_working_dir;
    let mut current_command: CommandLine;

    // ^([$a-z0-9]+)( .+)+
    let first_reg = Regex::new(r"^([$\w\d]+) ([\w\d\.]+)(.*)$").expect("Regex compilation failed.");
    for line in lines.into_iter().map(|x| x.unwrap()) {
        let caps = first_reg.captures(&line).expect("Applying Regex failed.");
    }
}


struct CommandLine {
    command: Command,
    // arg: &str
}

enum Command {
    List,
    ChangeDir
}

struct Directory {
    members: Vec<Member>
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum Member {
    Directory,
    File(u32)
}