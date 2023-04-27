
fn remove_every_second<T>(data: &mut Vec<T>) {
    for i in 0..data.len() / 2 {
        data.remove(i + 1);
    }
}

#[test]
fn test_remove_every_second() {
    let mut input = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
    remove_every_second(&mut input);

    let expected = vec![0, 2, 4, 6, 8];

    assert_eq!(input, expected);
}


pub enum MyOption<T> {
    Some(T),
    None,
}

impl<T> MyOption<T> {
    pub fn unwrap(self) -> T {
        match self {
            MyOption::Some(value) => value,
            MyOption::None => panic!("Expected Some(value), got None"),
        }
    }
}

#[test]
fn test_options() {
    let some = Some(42_usize);
    assert_eq!(some.unwrap(), 42);

    let my_some = MyOption::Some(42_usize);
    assert_eq!(my_some.unwrap(), 42);
}