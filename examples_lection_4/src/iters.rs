#[test]
fn test_iters_1() {
    for i in 0..10 {
        println!("{i}");
    }
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
    for i in TakeIter::new(QuadraticIter::new(10), 5) {
        println!("{i}");
    }
}

#[test]
fn test_iters_5() {
    let quads = (1..).map(|it| it * it);

    for i in quads.take(5) {
        println!("{i}");
    }
}
