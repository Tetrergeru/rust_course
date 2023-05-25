use std::{
    sync::{
        atomic::{self, Ordering},
        mpsc, Arc, Mutex,
    },
    thread,
};

#[test]
fn multithreading_0() {
    // let mut data = 0;

    // let mut handles = vec![];
    // for i in 1..100 {
    //     let t = thread::spawn(|| {
    //         data = i;
    //     });
    //     handles.push(t);
    // }

    // for handle in handles {
    //     handle.join().unwrap();
    // }

    // println!("{data}")
}

#[test]
fn multithreading_1() {
    thread::spawn(move || {
        for i in 1..10 {
            println!("{i}");
        }
    });
}

#[test]
fn multithreading_2() {
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("{i}");
        }
    });

    match handle.join() {
        Ok(()) => (),
        Err(_) => panic!("Expected Ok(())"),
    }
}

#[test]
fn multithreading_panic() {
    let handle = thread::spawn(move || {
        panic!();
    });

    match handle.join() {
        Ok(()) => panic!("Expected thread to panic"),
        Err(err) => println!("{err:?}"),
    }
}

#[test]
fn multithreading_3() {
    for i in 1..10 {
        thread::spawn(move || {
            println!("{i}");
        });
    }
}

#[test]
fn multithreading_4() {
    let mut handles = vec![];
    for i in 1..10 {
        handles.push(thread::spawn(move || {
            println!("{i}");
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn mutex_1() {
    let mutex = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    for i in 1..10 {
        let mutex = mutex.clone();
        let t = thread::spawn(move || {
            let mut r = mutex.lock().unwrap();
            if *r < i {
                *r = i;
            }
        });
        handles.push(t);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("value = {}", *mutex.lock().unwrap())
}

#[test]
fn atomics_1() {
    let value = Arc::new(atomic::AtomicI32::new(0));

    let mut handles = vec![];
    for i in 1..10 {
        let value = value.clone();
        let t = thread::spawn(move || {
            let _ = value.fetch_update(Ordering::Acquire, Ordering::Relaxed, |r| {
                if r < i {
                    Some(i)
                } else {
                    None
                }
            });
        });

        handles.push(t);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("value = {}", value.load(Ordering::SeqCst));
}

#[test]
fn chans_1() {
    let (s, r) = mpsc::channel::<i32>();

    for i in 1..10 {
        let s = s.clone();
        thread::spawn(move || {
            s.send(i).unwrap();
        });
    }

    let mut max = 0;
    for res in (1..10).map(|_| r.recv().unwrap()) {
        if res > max {
            max = res;
        }
    }

    println!("value = {max}");
}

#[test]
fn chans_2() {
    let (s, r) = mpsc::channel::<i32>();

    for i in 1..10 {
        let s = s.clone();
        thread::spawn(move || {
            s.send(i).unwrap();
        });
    }

    std::mem::drop(s);

    let mut max = 0;
    while let Ok(res) = r.recv() {
        println!("{res}");
        if res > max {
            max = res;
        }
    }

    println!("value = {max}");
}

pub struct Foo1 {}

impl Foo1 {
    pub fn new() -> Self {
        Self {}
    }
    pub fn work(&mut self) {}
}

#[test]
fn send_1() {
    let mut foo = Foo1::new();

    thread::spawn(move || {
        foo.work();
    })
    .join()
    .unwrap();
}

pub struct Foo2 {
    pub data: *const u8,
}

impl Foo2 {
    pub fn new() -> Self {
        Self {
            data: std::ptr::null::<u8>(),
        }
    }
    pub fn work(&mut self) {}
}

unsafe impl Send for Foo2{}

#[test]
fn send_2() {
    let mut foo = Foo2::new();

    thread::spawn(move || {
        foo.work();
    })
    .join()
    .unwrap();
}

#[test]
fn sync_1() {
    // let value = 42;

    // let reference = &value;
    // thread::spawn(move || {
    //     println!("{reference}");
    // })
    // .join()
    // .unwrap();
}

#[test]
fn sync_2() {
    const VALUE: i32 = 42;

    let reference = &VALUE;
    thread::spawn(move || {
        println!("{reference}");
    })
    .join()
    .unwrap();
}

pub struct Foo3 {
    pub ptr: *const u8,
}

impl Foo3 {
    pub const fn new() -> Self {
        Self {
            ptr: std::ptr::null(),
        }
    }
    pub fn work(&self) {}
}

unsafe impl Send for Foo3 {}
unsafe impl Sync for Foo3 {}

#[test]
fn sync_3() {
    const VALUE: Foo3 = Foo3::new();

    let reference = &VALUE;
    thread::spawn(move || {
        reference.work();
    })
    .join()
    .unwrap();
}
