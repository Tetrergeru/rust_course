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

//*
// Ошибка компиляции
// pub fn make_foo<'a>(x: i32) -> Foo<'a> {
//     Foo { x: &x }
// }
// */
pub fn index_vec<'a, T>(vec: &'a Vec<T>, idx: usize) -> &'a T {
    &vec[idx]
}

#[test]
fn test_index_vec() {
    let vec = vec![1, 2, 3, 4];
    assert_eq!(index_vec(&vec, 0), &1);
}

pub fn make_foo_correct<'a>(x: &'a i32) -> Foo<'a> {
    Foo { x }
}

struct Token<'a> {
    typ: u32,
    text: &'a str,
}

fn substring<'b>(string: &'b String) -> &'b str {
    &string[0..1]
}

pub fn choose<'a>(a: &'a i32, b: &'a i32, flag: bool) -> &'a i32 {
    if flag {
        a
    } else {
        b
    }
}

const B: i32 = 43;

fn bar(a: &i32) -> &i32 {
    choose(a, &B, true)
}

#[test]
fn test_make_foo() {
    let a = 42;
    bar(&a);
}
