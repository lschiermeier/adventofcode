use aoc_2024::*;


enum Tile {
    Obstacle,
    Free
}

enum Visited {
    Not,
    Times(u64)
}

fn main() {
    let src_name = file!();
    let input_path = gen_input_path(src_name, false);
    let src_map = Map2D::<u8>::new(read_ascii(input_path).expect("File not found."));
    
}
