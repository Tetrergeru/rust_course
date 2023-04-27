#[test]
fn test_borrows() {
    let mut a = 42;
    let borrow = &a;

    println!("borrowed {borrow}");

    let mut_borrow = &mut a;
    *mut_borrow = 43;

    println!("borrowed {a}");
}

pub struct Foo<'a> {
    pub x: &'a i32,
}

/* 
// Ошибка компиляции
pub fn make_foo<'a>(x: i32) -> Foo<'a> {
    Foo { x: &x }
}
// */

pub fn index_vec<'a, T>(vec: &'a Vec<T>, idx: usize) -> &'a T {
    &vec[idx]
}

#[test]
fn test_index_vec() {
    let vec = vec![1,2,3,4];
    assert_eq!(index_vec(&vec, 0), &1);
}

pub fn make_foo_correct<'a>(x: &'a i32) -> Foo<'a> {
    Foo { x }
}

#[test]
fn test_make_foo() {
    let foo = make_foo_correct(&42);
    assert_eq!(foo.x, &42)
}