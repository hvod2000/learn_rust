use std::io;
use std::io::*;

fn main() {
    let input = io::stdin().lock().lines()
        .flat_map(|l| l.unwrap().split(' ').map(String::from).collect::<Vec<_>>());
    for (i, word) in input.enumerate() {
        println!("word[{}] = \"{}\"", i, word);
    }
}
