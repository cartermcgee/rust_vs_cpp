use std::time::Instant;

fn main() {
    let sudoku_grid_easy: [[i32; 9]; 9] =
            [[6,0,9,0,8,0,0,2,0],
            [0,7,0,0,1,3,6,0,0],
            [0,3,2,6,0,9,0,0,0],
            [3,2,0,0,0,0,0,0,8],
            [0,5,7,0,2,0,3,1,0],
            [8,0,0,0,0,0,0,5,2],
            [0,0,0,8,0,4,7,6,0],
            [0,0,5,1,3,0,0,4,0],
            [0,1,0,0,7,0,8,0,5]];

    let sudoku_grid_medium: [[i32; 9]; 9] =
            [[8,0,0,0,0,9,1,0,4],
            [0,3,2,0,0,0,0,0,6],
            [5,9,0,0,8,0,0,0,0],
            [0,0,0,7,9,0,0,0,0],
            [2,0,0,4,5,6,0,0,1],
            [0,0,0,0,1,8,0,0,0],
            [0,0,0,0,4,0,0,8,5],
            [6,0,0,0,0,0,7,1,0],
            [3,0,1,8,0,0,0,0,2]];

    let sudoku_grid_hard: [[i32; 9]; 9] =
            [[0,0,7,0,2,0,3,0,0],
            [0,4,0,7,0,0,9,0,0],
            [0,1,0,0,3,0,0,0,7],
            [0,0,0,0,0,1,0,6,8],
            [0,5,0,4,0,6,0,2,0],
            [9,8,0,2,0,0,0,0,0],
            [1,0,0,0,4,0,0,9,0],
            [0,0,9,0,0,3,0,5,0],
            [0,0,8,0,1,0,6,0,0]];

    println!("\nSolving Easy Sudoku Puzzles...");
    let mut total_easy = 0.0;
    for _i in 0..25 {
        let time = Instant::now();
        solve(sudoku_grid_easy);
        let elapsed = time.elapsed().as_micros() as f64 / 1000.0;
        total_easy += elapsed;
    }
    println!("Average: {}ms", total_easy / 25.0);

    println!("\nSolving Medium Sudoku Puzzles...");
    let mut total_medium = 0.0;
    for _i in 0..25 {
        let time = Instant::now();
        solve(sudoku_grid_medium);
        let elapsed = time.elapsed().as_micros() as f64 / 1000.0;
        total_medium += elapsed;
    }
    println!("Average: {}ms", total_medium / 25.0);

    println!("\nSolving Hard Sudoku Puzzles...");
    let mut total_hard = 0.0;
    for _i in 0..25 {
        let time = Instant::now();
        solve(sudoku_grid_hard);
        let elapsed = time.elapsed().as_micros() as f64 / 1000.0;
        total_hard += elapsed;
    }
    println!("Average: {}ms", total_hard / 25.0);
}

// fn print_array(array: [[i32; 9]; 9]){
//     print!("\n");
//     for i in 0..9{
//         print!("[ ");
//         for j in 0..9{
//             print!("{} ", array[i][j])
//         }
//         println!("]");
//     }
// }

fn possible(sudoku_grid: [[i32; 9]; 9], y: usize, x: usize, n: usize) -> bool{
    for i in 0..9{
        if sudoku_grid[y][i] == n as i32|| sudoku_grid[i][x] == n as i32{ return false }
    }
    let x_square = (x / 3) * 3;
    let y_square = (y / 3) * 3;
    for i in 0..3 {
        for j in 0..3 {
            if sudoku_grid[y_square + i][x_square + j] == n as i32 { return false }
        }
    }
    return true
}

fn solve(mut sudoku_grid: [[i32; 9]; 9]){
    for y in 0..9 {
        for x in 0..9 {
            if sudoku_grid[y][x] == 0 {
                for n in 1..10 {
                    if possible(sudoku_grid, y, x, n) {
                        sudoku_grid[y][x] = n as i32;
                        solve(sudoku_grid);
                        sudoku_grid[y][x] = 0;
                    }
                }
                return;
            }
        }
    }
    //print_array(sudoku_grid);
}
