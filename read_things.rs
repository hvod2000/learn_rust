use std::io;
use std::io::*;

fn main() {
    let mut input = io::stdin().lock().lines().map(|l| l.unwrap())
        .flat_map(|l| l.split(' ').map(String::from).collect::<Vec<_>>());
    macro_rules! read {
        () => {input.next().unwrap().parse().unwrap()};
        ($t: tt) => {input.next().unwrap().parse::<$t>().unwrap()};
    }
    print!("Please, input a word, a number and a boolen value: ");
    stdout().flush().unwrap_or(());
    println!("string = {}", read!(String));
    println!("number = {}", read!(f64));
    println!("boolean = {}", read!(bool));
    for (i, word) in input.enumerate() {
        println!("word[{}] = \"{}\"", i + 1, word);
    }
}
