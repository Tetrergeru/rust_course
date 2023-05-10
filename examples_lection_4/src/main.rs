#![allow(clippy::while_let_loop)]
#![allow(dead_code)]

pub mod module_1;
pub mod module_2;

#[cfg(test)]
pub mod smart_ptrs;

#[cfg(test)]
pub mod iters;

pub mod module_0 {
    pub mod module_1 {
        pub mod module_2 {
            // Только прямые предки текущего модуля
            pub(in crate::module_0) fn foo() {}
        }
    }

    pub fn foo() {}
}

fn main() {

}
