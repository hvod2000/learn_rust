// disjoint set union
// O(1) for all reasonable operations

type Sets = (Vec<u32>, Vec<u8>);

fn disjoint_sets(n: usize) -> Sets {
    ((0..n as u32).collect(), vec![0; n])
}

fn get_set(sets: &mut Sets, u: usize) -> usize {
    if u as u32 != sets.0[u] {
        sets.0[u] = get_set(sets, sets.0[u] as usize) as u32;
    }
    sets.0[u] as usize
}

fn merge_sets(sets: &mut Sets, u: usize, v: usize) {
    let (mut u, mut v) = (get_set(sets, u), get_set(sets, v));
    if u != v {
        let (parent, rank) = sets;
        match rank[u].cmp(&rank[v]) {
            std::cmp::Ordering::Less => (u, v) = (v, u),
            std::cmp::Ordering::Equal => rank[u] += 1,
            _ => (),
        }
        parent[v] = u as u32;
    }
}

#[test]
fn test_the_thing() {
    let mut sets = disjoint_sets(5);
    merge_sets(&mut sets, 0, 1);
    merge_sets(&mut sets, 2, 3);
    merge_sets(&mut sets, 2, 4);
    assert_eq!(sets, (vec![0, 0, 2, 2, 2], vec![1, 0, 1, 0, 0]));
}

fn main() {
    let mut sets = disjoint_sets(10);
    (0..10).for_each(|u| {get_set(&mut sets, u);});
    println!("sets = {:?}", sets.0);
    (0..4).for_each(|u| merge_sets(&mut sets, u, u * u));
    println!("uniting every x with x^2");
    (0..10).for_each(|u| {get_set(&mut sets, u);});
    println!("sets = {:?}", sets.0);
    println!("ranks: {:?}", sets.1);
}
