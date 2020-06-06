# Map macro for Rust

This provides some map macros.

## Usage

Add the following to the dependencies section of Cargo.toml.
```toml
macro-map = { git = "https://github.com/masinc/macro-map-rust" }
```

### hash_map! btree_map!

`hash_map!` macro creates `std::collections::Hashmap`.
`btree_map!` macro creates `std::collections::Btreemap`.

* Example

    The separator can be space or comma.


    ```rust
    #[macro_use(hash_map!)]
    extern crate macro_map;

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
    #[macro_use(btree_map!)]
    extern crate macro_map;


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
    #[macro_use(map!)]
    extern crate macro_map;

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

* Example

    ```rust
    #[macro_use(map_for!)]
    extern crate macro_map;

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
