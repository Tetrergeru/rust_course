use std::alloc::{self, Layout};

pub struct Data {
    ptr: *mut i32,
    data: i32,
}

impl Data {
    pub fn new(v: i32) -> Self {
        let ptr = unsafe { alloc::alloc(Layout::new::<i32>()) as *mut i32 };
        unsafe { *ptr = v };
        Self {
            ptr,
            data: v,
        }
    }

    pub fn set(&mut self, v: i32) {
        unsafe { *self.ptr = v };
        self.data = v;
    }

    pub fn get(&self) -> i32 {
        if self.data != unsafe { *self.ptr } {
            panic!();
        }
        self.data
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            alloc::dealloc(self.ptr as *mut u8, Layout::new::<i32>());
        }
    }
}

pub fn do_sth(data: &mut Data) {
    data.set(43);
}

#[test]
fn test_data() {
    let mut data = Data::new(42);
    println!("{}", data.get());
    do_sth(&mut data);
    println!("{}", data.get());
}
