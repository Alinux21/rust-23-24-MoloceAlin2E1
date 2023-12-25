use std::fs;
use std::thread::sleep;
use std::time::Duration;
const MAX_ROWS: usize = 100;
const MAX_COLS: usize = 100;
type Matrix = [[char; MAX_COLS]; MAX_ROWS];
fn init_matrix(matrix: &mut Matrix) {
    let content = fs::read_to_string("src/gospergun.txt").unwrap();
    let mut i: usize = 0;
    let mut j: usize;
    for line in content.split('\n') {
        j = 0;
        for ch in line.chars() {
            matrix[i][j] = ch;
            j = j + 1;
        }
        i = i + 1;
    }
}
fn neighbours(m: Matrix, x: usize, y: usize) -> u8 {
    let mut live_neighbours: u8 = 0;

    for i in x - 1..x + 2 {
        for j in y - 1..y + 2 {
            if m[i][j] == 'x' {
                live_neighbours += 1;
            }
        }
    }
    if m[x][y] == 'x' {
        return live_neighbours - 1;
    } else {
        return live_neighbours;
    }
}
fn udpate_matrix(m: Matrix) -> Matrix {
    let mut updated_matrix: Matrix = [[' '; MAX_COLS]; MAX_ROWS];

    for i in 1..MAX_ROWS - 1 {
        for j in 1..MAX_COLS - 1 {
            if m[i][j] == 'x' && (neighbours(m, i, j) == 3 || neighbours(m, i, j) == 2) {
                //live cell
                updated_matrix[i][j] = 'x';
            } else if m[i][j] == ' ' && neighbours(m, i, j) == 3 {
                updated_matrix[i][j] = 'x';
            } else {
                updated_matrix[i][j] = ' ';
            }
        }
    }

    updated_matrix
}
fn print_matrix(m: Matrix) {
    for i in 0..20 {
        for j in 0..MAX_COLS {
            print!("{}", m[i][j]);
        }
        println!();
    }
}
fn main() {
    let mut matrix: Matrix = [[' '; MAX_COLS]; MAX_ROWS];
    init_matrix(&mut matrix);
    loop {
        let updated_matrix: Matrix = udpate_matrix(matrix);
        print_matrix(updated_matrix);
        println!("=== === === === === === === ===");
        matrix = updated_matrix;
        sleep(Duration::from_millis(50));
    }
}