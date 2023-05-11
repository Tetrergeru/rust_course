use std::{borrow::Cow, rc::Rc};

#[test]
fn test_no_box() {
    let data = [0_u8; 1024 * 1024];

    for i in data {
        if i > 0 {
            println!("{i} > 0");
        }
    }
}

#[test]
fn test_box() {
    let data = vec![0_u8; 1024 * 1024].into_boxed_slice();

    for i in data.iter() {
        if *i > 0 {
            println!("{i} > 0");
        }
    }
}

struct Cloneable(i32);

impl Clone for Cloneable {
    fn clone(&self) -> Self {
        println!("Cloneable({}) was cloned", self.0);
        Self(self.0)
    }
}

impl Drop for Cloneable {
    fn drop(&mut self) {
        println!("Cloneable({}) was dropped", self.0);
    }
}

fn consume_rc(rc: Rc<Cloneable>) {
    println!("Rc<Cloneable({})> was consumed", rc.0);
}

#[test]
fn test_rc() {
    let rc = Rc::new(Cloneable(42));

    consume_rc(rc.clone());
    consume_rc(rc.clone());
    consume_rc(rc);
}

fn modulo_3(input: u8) -> Cow<'static, str> {
    match input % 3 {
        0 => {
            let s = "Remainder is 0";
            s.into()
        }
        1 => "Remainder is 1".into(),
        remainder => {
            let s = format!("Remainder is {remainder}");
            s.into()
        }
    }
}

#[test]
fn test_cow() {
    for number in 1..=6 {
        match modulo_3(number) {
            Cow::Borrowed(message) => println!("{number} went in. The Cow is borrowed: {message}"),
            Cow::Owned(message) => println!("{number} went in. The Cow is owned: {message}"),
        }
    }
}
