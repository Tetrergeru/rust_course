use std::collections::VecDeque;

#[test]
fn test_iters_1() {
    for i in 0..10 {
        println!("{i}");
    }
}

enum MyOption<T> {
    Some(T),
    None,
}

#[test]
fn test_option_size() {
    println!("sizeof(*i32) = {}", std::mem::size_of::<*const i32>());
    println!("sizeof(&i32) = {}", std::mem::size_of::<&i32>());

    println!(
        "sizeof(Option<*i32>) = {}",
        std::mem::size_of::<MyOption<*const i32>>()
    );
    println!(
        "sizeof(Option<&i32>) = {}",
        std::mem::size_of::<MyOption<&i32>>()
    );
}

#[test]
fn test_iters_2() {
    // Развернём синтаксический сахар

    let mut iter = 0..10;

    loop {
        match iter.next() {
            Some(i) => {
                println!("{i}");
            }
            None => break,
        }
    }
}

struct QuadraticIter {
    i: usize,
    total: usize,
}

impl QuadraticIter {
    fn new(total: usize) -> Self {
        Self { total, i: 1 }
    }
}

impl Iterator for QuadraticIter {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        if self.i > self.total {
            return None;
        }

        let v = self.i;
        self.i += 1;

        Some(v * v)
    }
}

#[test]
fn test_iters_3() {
    for i in QuadraticIter::new(10) {
        println!("{i}");
    }
}

struct TakeIter<I, Iter>
where
    Iter: Iterator<Item = I>,
{
    inner: Iter,
    i: usize,
    total: usize,
}

impl<I, Iter> TakeIter<I, Iter>
where
    Iter: Iterator<Item = I>,
{
    fn new(inner: Iter, total: usize) -> Self {
        Self { inner, i: 0, total }
    }
}

impl<I, Iter> Iterator for TakeIter<I, Iter>
where
    Iter: Iterator<Item = I>,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i >= self.total {
            return None;
        }

        self.i += 1;
        self.inner.next()
    }
}

#[test]
fn test_iters_4() {
    for i in TakeIter::new(0.., 15) {
        println!("{i}");
    }
}

#[test]
fn test_iters_5() {
    let quads = (1..).map(|it| it * it).filter(|it| it % 2 == 0);

    for (idx, i) in quads.take(15).enumerate() {
        println!("{i} {idx}");
    }
}

#[test]
fn test_vec_iter() {
    let mut v = VecDeque::new();

    v.push_back(42);
    v.push_back(44);
    v.push_front(43);
    v.push_front(45);

    for k in v.iter() {
        println!("{k}");
    }
}
