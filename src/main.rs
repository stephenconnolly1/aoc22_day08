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
    pub score: i32,
}

fn main() {
    let mut grid: Vec<Vec<Tree>> = vec![vec![]];
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    populate_grid(&mut grid, file_path);    
    check_visibility(&mut grid);  
    measure_scenic_scores (&mut grid);
}
fn populate_grid(grid: &mut Vec<Vec<Tree>>, file_path: &String) {
    let mut row: usize = 0;
    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if row > 0 { 
                grid.push(vec![]);
            }
            if let Ok(x) = line {
                for c in x.chars() {
                    let h: u32 = c.to_digit(10).unwrap();
                    grid[row].push(Tree{height: h, visible: false, score:0});
                }
            }
            row = row + 1;
        }
    }
//    print!("{:?}", grid);

}
fn check_visibility(grid: &mut Vec<Vec<Tree>>) {
    let mut visible_count = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if is_visible(&grid, row, col) {
                grid[row][col].visible = true;
                visible_count = visible_count+1;
            }
        }
    }
    println!("Visible trees: {}", visible_count);
}
fn measure_scenic_scores(grid: &mut Vec<Vec<Tree>>){
    println!("Checking scenic score");
    let mut max_scenic_score = 0;
    let mut score: i32 = 0;
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            score = calc_score(&grid, row, col);
            grid[row][col].score = score;
            if grid[row][col].score > max_scenic_score {
                max_scenic_score = grid[row][col].score;
            }
        }
    }
    println!("max scenic score: {}", max_scenic_score);
}

fn is_visible(grid: &Vec<Vec<Tree>>, row: usize, col: usize) -> bool{
//    let mut v = false;
    let mut taller_than_others = true;
    let h = grid[row][col].height;
    // Check Perimeter
    if col == 0 { 
        println!("Col-0 = Visible");
        return true;
    }
    if col == grid[row].len() - 1 { 
        println!("Col-{} = Visible", col);
        return true;
    }
    if row == 0 { 
        println!("Row-0  = Visible");
        return true;
    }
    if row == grid.len() - 1 { 
        println!("Row-{}  = Visible", row);
        return true;
    }

    // check from West
    for i in 0..col {
        taller_than_others = taller_than_others && (h > grid[row][i].height) ;
    }
    if taller_than_others {
        println!("{},{} - Visible from West", row, col);
        return true;
    }
    // check from East
    taller_than_others = true;
    for i in col+1..grid[row].len() {
        taller_than_others = taller_than_others && (h > grid[row][i].height) ;
    }
    if taller_than_others {
        println!("{},{} - Visible from East", row, col);
        return true;
    }

    // check from North
    taller_than_others = true;
    for i in 0..row {
        taller_than_others = taller_than_others && (h > grid[i][col].height) ;
    }
    if taller_than_others {
        println!("{},{} - Visible from North", row, col);
        return true;
    }
    // check from South
    taller_than_others = true;
    for i in row+1..grid.len() {
        taller_than_others = taller_than_others && (h > grid[i][col].height) ;
    }
    if taller_than_others {
        println!("{},{} - Visible from South", row, col);
        return true;
    }
    // Not visible 
    println!("{},{} Not visible", row, col);
    return false;
}

fn calc_score(grid: &Vec<Vec<Tree>>, row: usize, col: usize) -> i32{
        let h = grid[row][col].height;
        println!("row:{}, col:{}, height:{}", row, col, h);
        // Check Perimeter
        if  col == 0 || 
            row == 0 || 
            col == grid[row].len() - 1 || 
            row == grid.len() - 1  { 
                println!("perimeter: Score 0");
                return 0;
            }
        // if col == 1 ||
        //     row == 1 ||
        //     col ==  grid[row].len() - 2 ||   
        //     row == grid.len() - 2  {
        //         println!("Inner perimeter: Score 1");
        //         return 1;                
        //     } 
        // check from West
        let mut west_distance = 0;
        for i in (0..col).rev() {
            west_distance = west_distance + 1;
            if h > grid[row][i].height {
                println!("h:{}, i:{}, height:{}", h, i, grid[row][i].height);
            } else {
                break;
            }
        }
        println!("Wdist:{}", west_distance);

        // check from East
        let mut east_distance = 0;
        for i in col + 1..grid[row].len() {
            east_distance = east_distance + 1;
            if h > grid[row][i].height {
                println!("h:{}, i:{}, height:{}", h, i, grid[row][i].height);
            } else {
                break;
            }
        }
        println!("Edist:{}", east_distance);
    
        // check from North
        let mut north_distance = 0;
        for i in (0..row).rev() {
            north_distance = north_distance + 1;
            if h > grid[i][col].height {
                println!("h:{}, i:{}, height:{}", h, i, grid[i][col].height);
            } else {
                break;
            }
        }
        println!("Ndist:{}", north_distance);

        // check from South
        let mut south_distance = 0;
        for i in row + 1..grid.len() {
            south_distance = south_distance + 1;
            if h > grid[i][col].height {
                println!("h:{}, i:{}, height:{}", h, i, grid[i][col].height);
            } else {
                break;
            }
        }
        println!("SDist:{}", south_distance);

        let score = north_distance*south_distance*east_distance*west_distance;
        println!("({},{}) Score:
        {}", row, col, score);
        return score;
    }
    
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn t1() {
        let mut grid: Vec<Vec<Tree>> = vec![vec![]];
        let file_path = "./src/test.txt".to_string();
        populate_grid(&mut grid, &file_path);    
        assert_eq!(grid.len(), 5);
        assert_eq!(grid[0].len(), 5);
        assert_eq!(grid[0][0].height, 3);
        check_visibility(&mut grid);  
        assert_eq!(grid[0][0].visible, true);
        assert_eq!(grid[1][1].visible, true);
        assert_eq!(grid[1][2].visible, true);
        assert_eq!(grid[1][3].visible, false);
        assert_eq!(grid[1][4].visible, true);


    }
    #[test]
    fn t2() {
        let mut grid: Vec<Vec<Tree>> = vec![vec![]];
        let file_path = "./src/test.txt".to_string();
        populate_grid(&mut grid, &file_path);    
        measure_scenic_scores(&mut grid);  
        assert_eq!(grid[3][2].score, 8);
    
    
    }
 
}