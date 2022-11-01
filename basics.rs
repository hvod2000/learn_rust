fn main() {
    let numbers = [1, 2, 3, 4, -5];
    let sums = numbers.iter().fold(vec![0], |mut vec, &x| {
        vec.push(x + vec.last().unwrap());
        vec
    });
    println!("       numbers: {:?}", numbers);
    println!("prefix sums: {:?}", sums);
}
