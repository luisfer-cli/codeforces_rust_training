/**
 A. Beautiful Year time limit per test
 2 seconds memory limit per test
 256 megabytes

 It seems like the year of 2013 came only yesterday. Do you know a curious fact? The year of 2013
 is the first year after the old 1987 with only distinct digits.

 Now you are suggested to solve the following problem: given a year number, find the minimum year
 number which is strictly larger than the given one and has only distinct digits. Input

 The single line contains integer y (1000 ≤ y ≤ 9000) — the year number. Output

 Print a single integer — the minimum year number that is strictly larger than y and all it's
 digits are distinct. It is guaranteed that the answer exists.
**/
use std::io::{self, Read};

fn has_different_digits(number: String) -> bool {
    for n in number.chars() {
        let repeats = number.chars().filter(|&x| x == n).count();
        if repeats > 1 {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut year = input.lines().next().unwrap().to_string();
    loop {
        let mut year_number: i32 = year.parse().unwrap();
        year_number = year_number + 1;
        year = year_number.to_string();
        if has_different_digits(year.clone()) {
            break;
        }
    }
    println!("{}", year);
}
