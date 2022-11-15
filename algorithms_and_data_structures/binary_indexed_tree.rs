// binary indexed tree with indexation starting from 1
// :::::::::::::::::::::16
// :::::::8               ::::::::::::::24
// :::4    :::::12        ::::::20        ::::::28
// :2  :6  :10    ::14    ::18    ::22    ::26    ::30
// 1 3 5 7 9  11  13  15  17  19  21  23  25  27  29  31
// navigation:
//   nearest_ansestor_to_the_left = k - msb(k)
//   nearest_ansestor_to_the_right = k + msb(k)
//   parent = (k - msb(k)) | (msb(k) << 1)
//   left child = k - msb(k) / 2
//   right child = k + msb(k) / 2
//   mbs(k) = 1 << k.trailing_zeros() - most significant bit
// usage:
//   1. Put in every node its prefix on subtree.
//   This way you get O(log(n)) querying of sum on range.
//   2. The same as (1), but values are prefix sums.
//   You get O(log(n)) querying of sum on range and
//   O(log(n)) adding value on range.
//
// binary indexed tree with indexation starting from 0
// ::::::::::::::::::::15
// :::::::7              ::::::::::::::23
// :::2    ::::11        ::::::19        ::::::27
// :1  :5  :9    ::13    ::17    ::21    ::25    ::29
// 0 2 4 6 8 10  12  14  16  18  20  22  24  26  28  30
// navigation:
//   the same as with indexing from 1, but instead of
//     msb(k) = 1 << k.trailing_zeros()
//   you should use
//     msz(k) = 1 << k.trailing_ones()

fn binary_indexed_tree(elements: Vec<i64>) -> Vec<i64> {
    let mut tree = vec![0; elements.len() + 1];
    for (mut value, i) in elements.into_iter().zip(1usize..) {
        let mut j = (1 << i.trailing_zeros()) / 2;
        while j > 0 {
            value += tree[i - j];
            j /= 2;
        }
        tree[i] = value;
    }
    tree
}

fn add(tree: &mut [i64], i: usize, delta: i64) {
    let mut i = i + 1;
    while i < tree.len() {
        tree[i] += delta;
        i += 1 << i.trailing_zeros();
    }
}

fn nth(tree: &[i64], mut i: usize) -> i64 {
    let (mut j, mut result) = (i + 1, 0);
    while i < j {
        result += tree[j];
        j -= 1 << j.trailing_zeros();
    }
    while i > j {
        result -= tree[i];
        i -= 1 << i.trailing_zeros();
    }
    result
}

#[test]
fn test_representation() {
    let tree = binary_indexed_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
    assert_eq!(tree, [0, 3, 4, 4, 9, 5, 14, 2, 31]);
}

#[test]
fn test_addition() {
    let mut tree = binary_indexed_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
    add(&mut tree, 4, -50);
    let vals = (1..tree.len()).map(|i| nth(&tree, i - 1)).collect::<Vec<_>>();
    assert_eq!(vals, [3, 1, 4, 1, -45, 9, 2, 6]);
}

fn main() {
    use std::io;
    use std::io::*;
    let mut input = io::stdin().lock().lines().map(|l| l.unwrap());
    let arr = input.next().unwrap().split(' ').map(|w| w.parse().unwrap()).collect();
    let mut tree = binary_indexed_tree(arr);
    println!("tree = {:?}", tree);
    println!("vals = {:?}", (1..tree.len()).map(|i| nth(&tree, i - 1)).collect::<Vec<_>>());
    println!("Decrementing second element by 42");
    add(&mut tree, 1, -42);
    println!("tree = {:?}", tree);
    println!("vals = {:?}", (1..tree.len()).map(|i| nth(&tree, i - 1)).collect::<Vec<_>>());
}
