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
//   2. The same as (1), but values are element differences.
//   You get O(log(n)) querying of value and
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
//
// imho indexation from 1 is slightly simpler both to understand
// and implement, but in general both of them are quite simple.

fn msz(i: usize) -> usize { 1 << i.trailing_ones() }

fn binary_indexed_tree(mut tree: Vec<i64>) -> Vec<i64> {
	let n = tree.len();
	(0..tree.len()).filter(|i| i + msz(*i) < n).for_each(|i| tree[i + msz(i)] += tree[i]);
	tree
}

fn add(tree: &mut [i64], mut i: usize, delta: i64) {
	while i < tree.len() {
		tree[i] += delta;
		i += msz(i);
	}
}

fn sum(tree: &[i64], first: usize, last: usize) -> i64 {
	let (mut i, mut j) = (first as isize - 1, last as isize);
	let mut result = 0;
	while i < j {
		result += tree[j as usize];
		j -= msz(j as usize) as isize;
	}
	while i > j {
		result -= tree[i as usize];
		i -= msz(i as usize) as isize;
	}
	result
}

fn nth(tree: &[i64], i: usize) -> i64 {
	let mut result = tree[i];
	let (mut i, j) = (i as isize - 1, i as isize - msz(i) as isize);
	while i > j {
		result -= tree[i as usize];
		i -= msz(i as usize) as isize;
	}
	result
}

#[test]
fn test_representation() {
	let tree = binary_indexed_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
	assert_eq!(tree, [3, 4, 4, 9, 5, 14, 2, 31]);
}

#[test]
fn test_restoring_elements() {
	let tree = binary_indexed_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
	let vals = (0..tree.len()).map(|i| nth(&tree, i)).collect::<Vec<_>>();
	assert_eq!(vals, [3, 1, 4, 1, 5, 9, 2, 6]);
}

#[test]
fn test_addition() {
	let mut tree = binary_indexed_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
	add(&mut tree, 4, -50);
	let vals = (0..tree.len()).map(|i| nth(&tree, i)).collect::<Vec<_>>();
	assert_eq!(vals, [3, 1, 4, 1, -45, 9, 2, 6]);
}

#[test]
fn test_summation() {
	let tree = binary_indexed_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
	assert_eq!(sum(&tree, 4, 6), 16);
	assert_eq!(sum(&tree, 0, 3), 9);
}

fn main() {
	use std::io::*;
	let mut input = stdin().lock().lines().map(|l| l.unwrap());
	let arr = input.next().unwrap().split(' ').map(|w| w.parse().unwrap()).collect();
	let mut tree = binary_indexed_tree(arr);
	println!("tree = {:?}", tree);
	println!("vals = {:?}", (0..tree.len()).map(|i| nth(&tree, i)).collect::<Vec<_>>());
	println!("Decrementing second element by 42");
	add(&mut tree, 1, -42);
	println!("tree = {:?}", tree);
	println!("vals = {:?}", (0..tree.len()).map(|i| nth(&tree, i)).collect::<Vec<_>>());
	println!("sums = {:?}", (0..tree.len()).map(|i| sum(&tree, 0, i)).collect::<Vec<_>>());
}
