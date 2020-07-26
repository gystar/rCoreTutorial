#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
extern crate alloc;
use alloc::{vec, vec::Vec};
use user_lib::sys_get_tid;

#[no_mangle]
pub fn main() -> usize {
    let mut vec = vec![1, 2, 3];
    println!("Hello world from user mode program!\n");
    0
}
