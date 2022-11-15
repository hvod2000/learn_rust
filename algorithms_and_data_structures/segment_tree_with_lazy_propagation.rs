// segment tree with lazy propagation
//
// supported operations:
//   O(log(n)) - set value of an element
//   O(log(n)) - get value of an element
//   O(log(n)) - lazily change values on range
//   O(log(n)) - get sum/max/min on range
//
// [:::::::::::::::0::::::::::::::::::]
// [::::::1:::::::] [::::::::2::::::::]
// [::3::] [::4:::] [:::5:::] [:::6:::]
// [7] [8] [9] [10] [11] [12] [13] [14] <- indexes of nodes
// {0} {1} {2} {3}  {4}  {5}  {6}  {7} <- indexes of stored elements
//
// navigation:
//   (k - 1) >> 1 - parent
//   k * 2 + 1    - first child
//   k * 2 + 2    - second child
//   i + (N >> 1) - index of element into index of node
//   k - (N >> 1) - node index to element's index
//   where N is a number of nodes in a tree
//     2n-1 - number of nodes in best case(n is power of two)
//     4n-5 - number of nodes in worst case(n = 2^k + 1)
//

fn segment_tree(elements: Vec<i64>) -> Vec<[i64; 2]> {
    let n = elements.len().next_power_of_two() * 2 - 1;
    let mut tree = vec![[0, 0]; n];
    for (i, value) in elements.into_iter().enumerate() {
        tree[n / 2 + i][0] = value;
    }
    for i in (0..(n / 2)).rev() {
        tree[i][0] = tree[i * 2 + 1][0] + tree[i * 2 + 2][0];
    }
    tree
}

fn set_nth(tree: &mut Vec<[i64; 2]>, i: usize, value: i64) {
    add(tree, i, i, value - nth(tree, i));
}

fn nth(tree: &Vec<[i64; 2]>, i: usize) -> i64 {
    let mut i = tree.len() / 2 + i;
    let mut value = tree[i][0] + tree[i][1];
    while i > 0 {
        i = (i - 1) >> 1;
        value += tree[i][1];
    }
    value
}

fn add(tree: &mut Vec<[i64; 2]>, first: usize, last: usize, delta: i64) {
    fn add(tree: &mut Vec<[i64; 2]>, first: usize, last: usize,
                i: usize, left: usize, right: usize, delta: i64) {
        if first <= right && left <= last {
            if first <= left && right <= last {
                tree[i][1] += delta;
            } else {
                let middle = (left + right) / 2;
                add(tree, first, last, i * 2 + 1, left, middle, delta);
                add(tree, first, last, i * 2 + 2, middle + 1, right, delta);
                tree[i][0] = tree[i * 2 + 1][0] + tree[i * 2 + 2][0];
                tree[i][0] += tree[i * 2 + 1][1] * (middle - left + 1) as i64;
                tree[i][0] += tree[i * 2 + 2][1] * (right - middle) as i64;
            }
        }
    }
    add(tree, first, last, 0, 0, tree.len() / 2, delta);
}

fn sum(tree: &Vec<[i64; 2]>, first: usize, last: usize) -> i64 {
    fn sum(tree: &Vec<[i64; 2]>, first: usize, last: usize,
            i: usize, left: usize, right: usize) -> i64 {
        if first > right || last < left {
            0
        } else if first <= left && right <= last {
            tree[i][0] + tree[i][1]
        } else {
            tree[i][1]
                + sum(tree, first, last, i * 2 + 1, left, (left + right) / 2)
                + sum(tree, first, last, i * 2 + 2, (left + right) / 2 + 1, right)
        }
    }
    sum(tree, first, last, 0, 0, tree.len() / 2)
}

#[test]
fn test_representation() {
    let tree = segment_tree(vec![3, 1, 4, 1]);
    assert_eq!(tree, [[9, 0], [4, 0], [5, 0], [3, 0], [1, 0], [4, 0], [1, 0]]);
}

#[test]
fn test_setter_and_getter() {
    let mut tree = segment_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
    set_nth(&mut tree, 2, -42);
    let elements: Vec<i64> = (0..8).map(|i| nth(&tree, i)).collect();
    assert_eq!(elements, [3, 1, -42, 1, 5, 9, 2, 6]);
}

#[test]
fn test_sum_on_range() {
    let tree = segment_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
    let sums: Vec<i64> = (2..8).map(|i| sum(&tree, i - 2, i)).collect();
    assert_eq!(sums, [8, 6, 10, 15, 16, 17]);
}

#[test]
fn test_update_on_range() {
    let mut tree = segment_tree(vec![3, 1, 4, 1, 5, 9, 2, 6]);
    add(&mut tree, 2, 5, -42);
    let elements: Vec<i64> = (0..8).map(|i| nth(&tree, i)).collect();
    assert_eq!(elements, [3, 1, -38, -41, -37, -33, 2, 6]);
}

fn main() {
    use std::io;
    use std::io::*;
    let input = io::stdin().lock().lines().next().unwrap().unwrap();
    let array: Vec<i64> = input.split(' ').map(|w| w.parse().unwrap()).collect();
    let n = array.len();
    let mut tree = segment_tree(array);
    println!("nodes = {:?}", tree);
    println!("elements = {:?}", (0..n).map(|i| nth(&tree, i)).collect::<Vec<_>>());
    println!("elements[2] = 42");
    set_nth(&mut tree, 2, 42);
    println!("elements = {:?}", (0..n).map(|i| nth(&tree, i)).collect::<Vec<_>>());
    println!("perfix sums: {:?}", (0..n).map(|i| sum(&tree, 0, i)).collect::<Vec<_>>());
    println!("nodes = {:?}", tree);
}
