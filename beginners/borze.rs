/**
  B. Borze time limit per test
  2 seconds memory limit per test
  256 megabytes

  Ternary numeric notation is quite popular in Berland. To telegraph the ternary number the Borze
  alphabet is used. Digit 0 is transmitted as «.», 1 as «-.» and 2 as «--». You are to decode the
  Borze code, i.e. to find out the ternary number given its representation in Borze alphabet. Input

  The first line contains a number in Borze code. The length of the string is between 1 and 200
  characters. It's guaranteed that the given string is a valid Borze code of some ternary number
  (this number can have leading zeroes). Output

  Output the decoded ternary number. It can have leading zeroes.
**/
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let string = input.lines().next().unwrap();
    let mut second = false;
    let mut result = String::new();
    for c in string.chars() {
        if c == '.' && !(second) {
            result.push('0');
        } else if c == '.' && second {
            result.push('1');
            second = false;
        } else if c == '-' && !(second) {
            second = true;
        } else {
            result.push('2');
            second = false;
        }
    }
    println!("{}", result);
}
