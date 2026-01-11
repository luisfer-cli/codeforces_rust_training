/**
 A. Lights Out time limit per test
 2 seconds memory limit per test
 256 megabytes

 Lenny is playing a game on a 3 × 3 grid of lights. In the beginning of the game all lights are
 switched on. Pressing any of the lights will toggle it and all side-adjacent lights. The goal of
 the game is to switch all the lights off. We consider the toggling as follows: if the light was
 switched on then it will be switched off, if it was switched off then it will be switched on.

 Lenny has spent some time playing with the grid and by now he has pressed each light a certain
 number of times. Given the number of times each light is pressed, you have to print the current
 state of each light. Input

 The input consists of three rows. Each row contains three integers each between 0 to 100
 inclusive. The j-th number in the i-th row is the number of times the j-th light of the i-th row
 of the grid is pressed. Output

 Print three lines, each containing three characters. The j-th character of the i-th line is "1"
 if and only if the corresponding light is switched on, otherwise it's "0".
**/
use std::io::{self, Read};

fn switch(number: i32) -> i32 {
    if number == 0 { 1 } else { 0 }
}

fn play(matrix: &mut Vec<Vec<i32>>, x: usize, y: usize) {
    matrix[x][y] = switch(matrix[x][y]);

    if x + 1 < 3 {
        matrix[x + 1][y] = switch(matrix[x + 1][y]);
    }
    if y + 1 < 3 {
        matrix[x][y + 1] = switch(matrix[x][y + 1]);
    }
    if x > 0 {
        matrix[x - 1][y] = switch(matrix[x - 1][y]);
    }
    if y > 0 {
        matrix[x][y - 1] = switch(matrix[x][y - 1]);
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut input_matrix: Vec<Vec<i32>> = vec![];
    for _ in 0..3 {
        let mut current_line = lines.next().unwrap().split_whitespace();
        let mut current_vec: Vec<i32> = vec![];

        for _ in 0..3 {
            current_vec.push(current_line.next().unwrap().parse().unwrap());
        }
        input_matrix.push(current_vec);
    }
    let mut matrix: Vec<Vec<i32>> = [[1, 1, 1], [1, 1, 1], [1, 1, 1]]
        .iter()
        .map(|row| row.to_vec())
        .collect();
    for x in 0..3 {
        for y in 0..3 {
            loop {
                if input_matrix[x][y] > 0 {
                    play(&mut matrix, x, y);
                    input_matrix[x][y] = input_matrix[x][y] - 1;
                } else {
                    break;
                }
            }
        }
    }
    for x in 0..3 {
        let mut output_line = String::new();
        for y in 0..3 {
            output_line.push_str(&(matrix[x][y].to_string()));
        }
        println!("{}", output_line);
    }
}
