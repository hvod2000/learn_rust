// Here is binary indexed tree that stores diffs of elements.
// O(log(n)) - getting an element by its index
// O(log(n)) - getting diffs of elements
// O(log(n)) - updating elements on range
//
// ::::::::::::::::::::15
// :::::::7              ::::::::::::::23
// :::2    ::::11        ::::::19        ::::::27
// :1  :5  :9    ::13    ::17    ::21    ::25    ::29
// 0 2 4 6 8 10  12  14  16  18  20  22  24  26  28  30

fn binary_indexed_tree(array: Vec<i64>) -> Vec<i64> {
	// TODO: O(n) construction if it is possible
	let mut previous = 0;
	let mut tree = vec![0; array.len()];
	for (i, mut value) in array.into_iter().enumerate() {
		(previous, value) = (value, value - previous);
		for j in 0..i.trailing_ones() {
			value += tree[i - (1 << j)];
		}
		tree[i] = value;
	}
	tree
}

fn nth(tree: &[i64], i: usize) -> i64 {
	let mut i = i as isize;
	let mut result = 0;
	while i >= 0 {
		result += tree[i as usize];
		i -= 1 << i.trailing_ones();
	}
	result
}

fn nth_diff(tree: &[i64], i: usize) -> i64 {
	let (mut i, mut j) = (i as isize - 1, i as isize);
	let mut result = 0;
	result += tree[j as usize];
	j -= 1 << j.trailing_ones();
	while i > j {
		result -= tree[i as usize];
		i -= 1 << i.trailing_ones();
	}
	result
}

fn add(tree: &mut [i64], i: usize, delta: i64) {
	tree[i] += delta;
	let mut j = i + 1;
	while j < tree.len() && j < i + (1 << i.trailing_ones()) {
		tree[j] -= delta;
		j += 1 << j.trailing_ones();
	}
}

fn add_range(tree: &mut [i64], start: usize, end: usize, delta: i64) {
	let (mut i, mut j) = (start, end + 1);
	while i < j {
		tree[i] += delta;
		i += 1 << i.trailing_ones();
	}
	while j < i && j < tree.len() {
		tree[j] -= delta;
		j += 1 << j.trailing_ones();
	}
}

#[test]
fn test_representation() {
	let tree = binary_indexed_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
	assert_eq!(tree, [3, 1, 3, 1, 4, 8, -7, 6]);
}

#[test]
fn test_restoring_elements() {
	let tree = binary_indexed_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
	let vals = (0..tree.len()).map(|i| nth(&tree, i)).collect::<Vec<_>>();
	assert_eq!(vals, [3, 1, 4, 1, 5, 9, 2, 6]);
}

#[test]
fn test_restoring_diffs() {
	let tree = binary_indexed_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
	let vals = (0..tree.len()).map(|i| nth_diff(&tree, i)).collect::<Vec<_>>();
	assert_eq!(vals, [3, -2, 3, -3, 4, 4, -7, 4]);
}

#[test]
fn test_addition() {
	let mut tree = binary_indexed_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
	add(&mut tree, 4, -50);
	let vals = (0..tree.len()).map(|i| nth(&tree, i)).collect::<Vec<_>>();
	assert_eq!(vals, [3, 1, 4, 1, -45, 9, 2, 6]);
}

#[test]
fn test_addition_on_range() {
	let mut tree = binary_indexed_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
	add_range(&mut tree, 4, 6, -50);
	let vals = (0..tree.len()).map(|i| nth(&tree, i)).collect::<Vec<_>>();
	assert_eq!(vals, [3, 1, 4, 1, -45, -41, -48, 6]);
}

#[test]
fn test_addition_on_hacky_range() {
	let mut tree = binary_indexed_tree(vec![3, 1, 4, 1, 5, 9]);
	add_range(&mut tree, 4, 5, -50);
	let vals = (0..tree.len()).map(|i| nth(&tree, i)).collect::<Vec<_>>();
	assert_eq!(vals, [3, 1, 4, 1, -45, -41]);
}

fn main() {
	use std::io;
	use std::io::*;
	let mut input = io::stdin().lock().lines().map(|l| l.unwrap());
	let arr = input.next().unwrap().split(' ').map(|w| w.parse().unwrap()).collect();
	let mut tree = binary_indexed_tree(arr);
	println!("nodes  = {:?}", tree);
	println!("values = {:?}", (0..tree.len()).map(|i| nth(&tree, i)).collect::<Vec<_>>());
	println!("diffs  = {:?}", (0..tree.len()).map(|i| nth_diff(&tree, i)).collect::<Vec<_>>());
	println!("Decrementing second element by 42");
	add(&mut tree, 1, -42);
	println!("nodes  = {:?}", tree);
	println!("values = {:?}", (0..tree.len()).map(|i| nth(&tree, i)).collect::<Vec<_>>());
	println!("diffs  = {:?}", (0..tree.len()).map(|i| nth_diff(&tree, i)).collect::<Vec<_>>());
	println!("Incrementing first three elements by 100");
	add_range(&mut tree, 0, 2, 100);
	println!("nodes  = {:?}", tree);
	println!("values = {:?}", (0..tree.len()).map(|i| nth(&tree, i)).collect::<Vec<_>>());
	println!("diffs  = {:?}", (0..tree.len()).map(|i| nth_diff(&tree, i)).collect::<Vec<_>>());
}
