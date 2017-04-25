// Sudoku Solution Finder
// Uses a brute force backtracking algorithm to find and print solutions

// Print the board
fn print(ref board: [[u8; 9]; 9]) {
    for r in 0..9 {
        for c in 0..9 {
            print!{"{}", board[r][c]};
        }
        println!{""};
    }
    println!{""};
}

// Check to see if we can put val in the row and col
fn okay(ref board: [[u8; 9]; 9], row: usize, col: usize, val: u8) -> bool {

    // Variables to assist with finding which 3x3 superblock we are in
    let box_r: usize = (row / 3) * 3;
    let box_c: usize = (col / 3) * 3;

    // Look vertically, horizontally, and in our 3x3 grid for the same number
    // Compiler optimizes this better than searching until row/col
    for i in 0..9 {
        if board[i][col] == val ||
            board[row][i] == val ||
            board[box_r + i / 3][box_c + i % 3] == val {
            return false;
        }
    }
    return true;
}

// Recursively fill in each sequential position
// Returns the number of solutions found
fn find(ref input: [[u8; 9]; 9], ref mut board: [[u8; 9]; 9],
        row: usize, col: usize) -> u64 {
    let mut find_row: usize = row;
    let mut find_col: usize = col + 1;

    // If we have found a solution then print it
    if find_row >= 9 {
        print(*board);
        return 1;
    }

    // If we are done with this column, start again in the next one
    if find_col >= 9 {
        find_row += 1;
        find_col = 0;
    }

    // If the source grid already has a number at this location, use it
    if input[row][col] > 0 {
        return find(*input, *board, find_row, find_col);
    }
    
    // Iterate through each potential value we can put at this location
    let mut n_solutions = 0;
    for val in 1..10 {
        if okay(*board, row, col, val) {
            board[row][col] = val;
            n_solutions += find(*input, *board, find_row, find_col);
        }
        board[row][col] = 0;
    }
    return n_solutions;
}

// Main function takes an example grid
fn main() {
    let input: [[u8; 9]; 9] = [[3, 0, 6, 5, 0, 8, 4, 0, 0],
                               [5, 2, 0, 0, 0, 0, 0, 0, 0],
                               [0, 8, 7, 0, 0, 0, 0, 3, 1],
                               [0, 0, 3, 0, 1, 0, 0, 8, 0],
                               [9, 0, 0, 8, 6, 3, 0, 0, 5],
                               [0, 5, 0, 0, 9, 0, 6, 0, 0],
                               [1, 3, 0, 0, 0, 0, 2, 5, 0],
                               [0, 0, 0, 0, 0, 0, 0, 7, 4],
                               [0, 0, 5, 0, 0, 0, 0, 0, 0]];
    let board: [[u8; 9]; 9] = input;
    let n: u64 = find(input, board, 0, 0);
    println!("There are {} solutions.", n);
}
