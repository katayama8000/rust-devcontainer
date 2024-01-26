use std::collections::BTreeSet;
pub fn run() {
    println!(
        "{}{}{}btree_set.rs{}{}{}",
        "🦀", "🦀", "🦀", "🦀", "🦀", "🦀"
    );

    let mut set = BTreeSet::new();
    set.insert(1);
    set.insert(2);
    // append
    set.append(&mut BTreeSet::new());
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
