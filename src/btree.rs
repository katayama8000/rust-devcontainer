use std::collections::{BTreeMap, BTreeSet};
pub fn run() {
    println!(
        "{}{}{}btree_set.rs{}{}{}",
        "🦀", "🦀", "🦀", "🦀", "🦀", "🦀"
    );

    let mut set = BTreeSet::new();
    set.insert(3);
    set.insert(4);
    set.insert(1);
    println!("{:?}", set);
    // append
    let mut set2: BTreeSet<i32> = BTreeSet::new();
    set2.insert(1);
    set2.insert(2);
    println!("{:?}", set2);
    set.append(&mut set2);
    println!("{:?}", set);
    // contains
    println!("{}", set.contains(&1));
    // len
    println!("{}", set.len());
    // is_empty
    println!("{}", set.is_empty());
    // remove
    set.remove(&1);
    println!("{:?}", set);
    // clear
    set.clear();
    println!("{:?}", set);
    // BTreeMap
    let mut map = BTreeMap::new();
    map.insert(3, "c");
    map.insert(1, "a");
    map.insert(2, "b");

    println!("{:?}", map);
}

// test
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        let mut set = BTreeSet::new();
        set.insert(1);
        set.insert(2);
        assert_eq!(set.contains(&1), true);
        assert_eq!(set.len(), 2);
        set.append(&mut BTreeSet::new());
    }
}
