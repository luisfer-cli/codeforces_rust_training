/**
A. Beautiful Matrix time limit per test
2 seconds memory limit per test
256 megabytes

You've got a 5 × 5 matrix, consisting of 24 zeroes and a single number one. Let's index the matrix
rows by numbers from 1 to 5 from top to bottom, let's index the matrix columns by numbers from 1 to
5 from left to right. In one move, you are allowed to apply one of the two following
transformations to the matrix:

Swap two neighboring matrix rows, that is, rows with indexes i and i + 1 for some integer i
(1 ≤ i < 5). Swap two neighboring matrix columns, that is, columns with indexes j and j + 1 for
some integer j (1 ≤ j < 5).

You think that a matrix looks beautiful, if the single number one of the matrix is located in its
middle (in the cell that is on the intersection of the third row and the third column). Count the
minimum number of moves needed to make the matrix beautiful. Input

The input consists of five lines, each line contains five integers: the j-th integer in the i-th
line of the input represents the element of the matrix that is located on the intersection of the
i-th row and the j-th column. It is guaranteed that the matrix consists of 24 zeroes and a single
number one. Output

Print a single integer — the minimum number of moves needed to make the matrix beautiful.
**/
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let mut x: i8 = 0;
    let mut y: i8 = 0;
    let mut finish: bool = false;
    for row in 1..6 {
        let mut current = lines.next().unwrap().split_whitespace();
        for column in 1..6 {
            let element: i8 = current.next().unwrap().parse().unwrap();
            if element == 1 {
                x = row;
                y = column;
                finish = true;
                break;
            }
        }
        if finish {
            break;
        }
    }
    let resultx = (x - 3).abs();
    let resulty = (y - 3).abs();
    let result = resulty + resultx;
    println!("{}", result);
}
