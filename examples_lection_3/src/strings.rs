#[test]
fn test_str_1() {
    let s = "test";
    let bytes = s.as_bytes();

    for byte in bytes {
        println!("{}", *byte as char);
    }
}

#[test]
fn test_str_2() {
    let s = "a";
    let bytes = s.as_bytes();
    assert_eq!(bytes.len(), 1);

    assert_eq!(std::mem::size_of::<char>(), 4);
}

#[test]
fn test_str_3() {
    let s = "тест";
    let bytes = s.as_bytes();

    for byte in bytes {
        println!("{}", *byte as char);
    }

    for char in s.chars() {
        println!("{char}");
    }
}

#[test]
fn test_str_4() {
    let s = "👦🏻";

    for char in s.chars() {
        println!("{char}");
    }
}

#[test]
fn test_string_1(){
    let _s = "test".to_string();
    let _s = "test".to_owned();
    let _s: String = "test".into();
}

#[test]
fn test_string_2(){
    let static_str = "test count 0 (result should not be 0!)";
    let s = static_str.to_string();

    assert_eq!(count_0s_1(&s), 2);
    assert_eq!(count_0s_2(&s), 2);
    // assert_eq!(count_0s_3(&s), 2); // Ошибка компиляции
    assert_eq!(count_0s_3(static_str), 2);
}

fn count_0s_1(s: &String) -> usize {
    let mut count = 0;
    for ch in s.chars() {
        count += (ch == '0') as usize;
    }
    count
}

fn count_0s_2(s: &str) -> usize {
    let mut count = 0;
    for ch in s.chars() {
        count += (ch == '0') as usize;
    }
    count
}

fn count_0s_3(s: &'static str) -> usize {
    let mut count = 0;
    for ch in s.chars() {
        count += (ch == '0') as usize;
    }
    count
}