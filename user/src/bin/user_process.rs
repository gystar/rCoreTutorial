#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::console::*;
use user_lib::sys_get_tid;

#[no_mangle]
pub fn main() -> ! {
    loop {
        for i in 0..u128::MAX {
            if i % 1000000 == 0 {
                let (tid, pid) = sys_get_tid();
                println!("[process {},thread {}]user_process ticks {}.", pid, tid, i);
            }
        }
    }
}
