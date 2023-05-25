use std::{collections::HashSet, hash::Hash};

pub fn unique<T: Eq>(data: &[T]) -> Vec<&'_ T> {
    let mut res = vec![];

    'outer: for d in data {
        for r in res.iter() {
            if *r == d {
                continue 'outer;
            }
        }

        res.push(d);
    }

    res
}

#[test]
fn test_unique() {
    let v = vec![1, 2, 3, 4, 42, 42, 3, 4, 5];
    let expected = vec![&1, &2, &3, &4, &42, &5];

    assert_eq!(unique(&v), expected);
}

pub fn fast_unique<T>(data: &[T]) -> Vec<&T>
where
    T: Eq + Hash,
{
    let mut set = HashSet::new();
    let mut res = vec![];

    for d in data {
        if set.contains(d) {
            continue;
        }

        res.push(d);
        set.insert(d);
    }

    res
}

#[test]
fn test_fast_unique() {
    let v = vec![1, 2, 3, 4, 42, 42, 3, 4, 5];
    let expected = vec![&1, &2, &3, &4, &42, &5];

    assert_eq!(fast_unique(&v), expected);
}

pub trait Constructable1<T> {
    fn construct(t: T) -> Self;
}

pub trait Constructable2 {
    type T;

    fn construct(t: Self::T) -> Self;
}

struct Foo(i64);

impl Constructable2 for Foo {
    type T = i64;

    fn construct(t: Self::T) -> Self {
        Self(t)
    }
}

// Ошибка компиляции
// impl Constructable2 for Foo {
//     type T = i32;

//     fn construct(t: Self::T) -> Self {
//         Self(t as i64)
//     }
// }

impl Constructable1<i64> for Foo {
    fn construct(t: i64) -> Self {
        Self(t)
    }
}

impl Constructable1<i32> for Foo {
    fn construct(t: i32) -> Self {
        Self(t as i64)
    }
}

impl<T, Type: From<T>> Constructable1<T> for Type {
    fn construct(t: T) -> Self {
        t.into()
    }
}

// Ошибка компиляции
// impl<C, Type> Constructable2 for Type
// where
//     Type: Default,
// {
//     type T = C;

//     fn construct(_t: Self::T) -> Self {
//         Self::default()
//     }
// }

pub fn construct_1<T, C: Constructable1<T>>(t: T) -> C {
    C::construct(t)
}

#[test]
fn test_consturct_1() {
    let c = construct_1::<i16, i32>(10_i16);

    assert_eq!(c, 10_i32);
}

pub fn construct_2<C: Constructable2>(t: C::T) -> C {
    C::construct(t)
}

#[test]
fn test_consturct_2() {
    let c = construct_2::<Foo>(10);

    assert_eq!(c.0, 0);
}

pub trait SomeTrait<'a> {
    type Output;
}

pub struct Bar<'a, T>
// Без этих строк не компилируется
where
    T: 'a,
{
    pub foo: <T as SomeTrait<'a>>::Output,
}

impl<'a, T> SomeTrait<'a> for T
where
    T: 'a,
{
    type Output = u32;
}

pub fn foo<'a>(a: &'a i32, b: &'a i32, f: bool) -> &'a i32 {
    if f {
        a
    } else {
        b
    }
}

pub fn bar(ref_b: &i32) {
    let a = 42;
    let ref_a = &a;
    foo(ref_a, ref_b, true);
}
