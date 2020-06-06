# Map macro for Rust

This provides some map macros.

## Usage

### hash_map! btree_map!

`hash_map!` macro creates `std::collections::Hashmap`.
`btree_map!` macro creates `std::collections::Btreemap`.

* Example

    The separator can be space or comma.


    ```rust
    let map = hash_map!("a" => 1, "b" => 2, "c" => 3);
    let map = hash_map!(
        "a" => 1
        "b" => 2
        "c" => 3
    );
    ```

    These are the same results as below.

    ```rust
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert("a", 1);
    map.insert("b", 2);
    map.insert("c", 3);
    ```

    The usage of `btree_map!` is the same for `hash_map!`.

    ```rust
    let map = btree_map!("a" => 1, "b" => 2, "c" => 3);
    let map = btree_map!(
        "a" => 1
        "b" => 2
        "c" => 3
    );
    ```

### map!

`map!` macro creates a map in the Rust context.

* Example

    ```rust
    use std::collections::{HashMap, BTreeMap};

    //create HashMap
    let map: HashMap<_, _> =  map!("a" => 1, "b" => 2, "c" => 3);

    let map: HashMap<_, _> =  map!(
        "a" => 1
        "b" => 2
        "c" => 3
    );

    // create BTreeMap
    let map: BtreeMap<_, _> = map!("a" => 1, "b" => 2, "c" => 3);

    let map: BtreeMap<_, _> = map!(
        "a" => 1
        "b" => 2
        "c" => 3
    );

    ```

### map_for!

`map_for!` macro creates a map of the specified type.

* Usage

    ```rust
    use std::collections::{HashMap, BTreeMap};

    //create HashMap
    let map = map_for!(HashMap<_,_>;
        "a" => 1, "b" => 2, "c" => 3
    );

    let map = map_for!(HashMap<_,_>;
        "a" => 1
        "b" => 2
        "c" => 3
    );

    //create BTreeMap
    let map = map_for!(BTreeMap<_,_>;
        "a" => 1, "b" => 2, "c" => 3
    );

    let map = map_for!(BTreeMap<_,_>;
        "a" => 1
        "b" => 2
        "c" => 3
    );

    ```
