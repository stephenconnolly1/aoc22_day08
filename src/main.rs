use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
#[derive(Debug)]
struct Tree {
    pub height: u32,
    pub visible: bool,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let mut grid: Vec<Vec<Tree>> = vec![vec![]];
    let mut row: usize = 0;
    let mut col: usize = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(x) = line {
                for c in x.chars() {
                    let h: u32 = c.to_digit(10).unwrap();
                    grid[row].push(Tree{height: h, visible: false});
                }
            }
            row = row + 1;
            grid.push(vec![]);
        }
    }
    print!("{:?}", grid);
}