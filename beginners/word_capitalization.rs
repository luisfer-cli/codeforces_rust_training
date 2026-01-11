/**
 A. Word Capitalization time limit per test
 2 seconds memory limit per test
 256 megabytes

 Capitalization is writing a word with its first letter as a capital letter. Your task is to
 capitalize the given word.

 Note, that during capitalization all the letters except the first one remains unchanged. Input

 A single line contains a non-empty word. This word consists of lowercase and uppercase English
 letters. The length of the word will not exceed 103. Output

 Output the given word after capitalization.
**/
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let word = input.lines().next().unwrap();
    let mut chars = word.chars();
    let mut first_letter: char = chars.next().unwrap();
    first_letter = first_letter.to_uppercase().next().unwrap();
    let result = format!("{}{}", first_letter, chars.collect::<String>());
    println!("{}", result);
}
