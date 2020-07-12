//! # 全局属性
//! - `#![no_std]`  
//!   禁用标准库
#![no_std]

//!
//! - `#![no_main]`  
//!   不使用 `main` 函数等全部 Rust-level 入口点来作为程序入口，不用常规的入口点
#![no_main]

/// 覆盖 crt0 中的 _start 函数
/// 我们暂时将它的实现为一个死循环
#[no_mangle] //禁用编译期间的名称重整（Name Mangling），保证生成命为_start的函数
pub extern "C" fn _start() -> ! {
    loop {}
}

/// 当 panic 发生时会调用该函数
/// 我们暂时将它的实现为一个死循环
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
