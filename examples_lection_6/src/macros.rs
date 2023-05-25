use proc_examples::my_println;

#[macro_export]
macro_rules! hash_map_1 {
    () => {
        std::collections::HashMap::new()
    };
}

#[test]
fn test_hash_map_1() {
    let mut hash = hash_map_1![];

    hash.insert(42, "Hello");

    assert_eq!(hash.len(), 1);
    assert_eq!(hash[&42], "Hello");
}

#[macro_export]
macro_rules! hash_map_2 {
    () => {
        std::collections::HashMap::new()
    };
    ( [ $key:expr ] = $val:expr ) => {
        {
            let mut map = std::collections::HashMap::new();
            map.insert($key, $val);
            map
        }
    };
}

#[test]
fn test_hash_map_2() {
    let hash = hash_map_2!{
        [42] = "Hello"
    };

    assert_eq!(hash.len(), 1);
    assert_eq!(hash[&42], "Hello");
}

#[macro_export]
macro_rules! hash_map_3 {
    () => {
        std::collections::HashMap::new()
    };
    ( $( [ $key:expr ] = $val:expr ),* ) => {
        {
            let mut map = std::collections::HashMap::new();
            $(
                map.insert($key, $val);
            )*
            map
        }
    };
}

#[test]
fn test_hash_map_3() {
    let hash = hash_map_3!{
        [42] = "Hello",
        [43] = "olleH"
    };

    assert_eq!(hash.len(), 2);
    assert_eq!(hash[&42], "Hello");
    assert_eq!(hash[&43], "olleH");
}

#[test]
fn test_procedural_macro() {
    my_println!("foo * bar"); 
}