#[macro_export]
macro_rules! hash_map {
    ($($k: expr => $v: expr)*) => {
        [$(($k, $v)),*].iter().cloned().collect::<std::collections::HashMap<_,_>>()
    };
    ($($k: expr => $v: expr),*) => { hash_map!($($k => $v)*) };
}

#[macro_export]
macro_rules! btree_map {
    ($($k: expr => $v: expr)*) => {
        [$(($k, $v)),*].iter().cloned().collect::<std::collections::BTreeMap<_,_>>()
    };
    ($($k: expr => $v: expr),*) => { btree_map!($($k => $v)*) };
}

#[macro_export]
macro_rules! map {
    ($($k: expr => $v: expr)*) => {
        [$(($k, $v)),*].iter().cloned().collect()
    };
    ($($k: expr => $v: expr),*) => { map!($($k => $v)*)};
}

#[macro_export]
macro_rules! map_for {
    ($t: ty; $($k: expr => $v: expr)*) => {
        [$(($k, $v)),*].iter().cloned().collect::<$t>()
    };
    ($t: ty; $($k: expr => $v: expr),*) => { map_for!($t; $($k => $v)*)};
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    fn hash_map_123<'a>() -> HashMap<&'a str, i32> {
        let mut map = HashMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        map
    }

    fn btree_map_123<'a>() -> BTreeMap<&'a str, i32> {
        let mut map = BTreeMap::new();
        map.insert("a", 1);
        map.insert("b", 2);
        map.insert("c", 3);
        map
    }

    #[test]
    fn test_hash_map() {
        let right = hash_map_123();
        assert_eq!(hash_map!("a" => 1 "b" => 2 "c" => 3), right);
        assert_eq!(hash_map!("a" => 1, "b" => 2, "c" => 3), right);
    }

    #[test]
    fn test_btree_map() {
        let right = btree_map_123();
        assert_eq!(btree_map!("a" => 1 "b" => 2 "c" => 3), right);
        assert_eq!(btree_map!("a" => 1, "b" => 2, "c" => 3), right);
    }

    #[test]
    fn test_map_hash_map() {
        let right = hash_map_123();

        // separator for space
        let left: HashMap<&str, i32> = map!("a" => 1, "b" => 2, "c" => 3);
        assert_eq!(left, right);
        let left: HashMap<_, _> = map!("a" => 1, "b" => 2, "c" => 3);
        assert_eq!(left, right);

        // separator for comma
        let left: HashMap<&str, i32> = map!("a" => 1 "b" => 2 "c" => 3);
        assert_eq!(left, right);
        let left: HashMap<_, _> = map!("a" => 1 "b" => 2 "c" => 3);
        assert_eq!(left, right);
    }

    #[test]
    fn test_map_btree_map() {
        let right = btree_map_123();

        // separator for space
        let left: BTreeMap<&str, i32> = map!("a" => 1 "b" => 2 "c" => 3);
        assert_eq!(left, right);
        let left: BTreeMap<_, _> = map!("a" => 1 "b" => 2 "c" => 3);
        assert_eq!(left, right);

        // separator for comma
        let left: BTreeMap<&str, i32> = map!("a" => 1, "b" => 2, "c" => 3);
        assert_eq!(left, right);
        let left: BTreeMap<_, _> = map!("a" => 1, "b" => 2, "c" => 3);
        assert_eq!(left, right);
    }

    #[test]
    fn test_map_for_hash_map() {
        let right = hash_map_123();

        // separator for space
        assert_eq!(
            map_for!(HashMap<&str, i32>; "a" => 1 "b" => 2 "c" => 3),
            right
        );
        assert_eq!(
            map_for!(HashMap<_, _>; "a" => 1 "b" => 2 "c" => 3),
            right
        );

        // separator for comma
        assert_eq!(
            map_for!(HashMap<&str, i32>; "a" => 1, "b" => 2, "c" => 3),
            right
        );
        assert_eq!(
            map_for!(HashMap<_, _>; "a" => 1, "b" => 2, "c" => 3),
            right
        );
    }

    #[test]
    fn test_map_for_btree_map() {
        let right = btree_map_123();

        // sperator for space
        assert_eq!(
            map_for!(BTreeMap<&str, i32>; "a" => 1 "b" => 2 "c" => 3),
            right
        );
        assert_eq!(
            map_for!(BTreeMap<_, _>; "a" => 1 "b" => 2 "c" => 3),
            right
        );

        // separator for comma
        assert_eq!(
            map_for!(BTreeMap<&str, i32>; "a" => 1, "b" => 2, "c" => 3),
            right
        );
        assert_eq!(
            map_for!(BTreeMap<_, _>; "a" => 1, "b" => 2, "c" => 3),
            right
        );
    }
}
