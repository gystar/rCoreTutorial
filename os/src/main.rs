//! # 全局属性
//!
//! - `#![no_std]`  
//!   禁用标准库
#![no_std]

//!
//! - `#![no_main]`  
//!   不使用 `main` 函数等全部 Rust-level 入口点来作为程序入口，不用常规的入口点
#![no_main]

//! # 一些 unstable 的功能需要在 crate 层级声明后才可以使用
//!
//! - `#![feature(llvm_asm)]`  
//!   内嵌汇编
#![feature(llvm_asm)]
//!
//! - `#![feature(global_asm)]`
//!   内嵌整个汇编文件
#![feature(global_asm)]
//!
//! - `#![feature(panic_info_message)]`  
//!   panic! 时，获取其中的信息并打印
#![feature(panic_info_message)]
//!
//! - `#![feature(alloc_error_handler)]`
//!   我们使用了一个全局动态内存分配器，以实现原本标准库中的堆内存分配。
//!   而语言要求我们同时实现一个错误回调，这里我们直接 panic
#![feature(alloc_error_handler)]
//!
//! - `#![feature(slice_fill)]`
//!   允许将 slice 填充值
#![feature(slice_fill)]

#[macro_use]
mod console;
mod interrupt;
mod memory;
mod panic;
mod process;
mod sbi;

extern crate alloc;
use process::*;
use spin::RwLock;

use alloc::sync::Arc;

// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("entry.asm"));

/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle] //禁用编译期间的名称重整（Name Mangling），保证生成命为_start的函数
pub extern "C" fn rust_main() -> ! {
    println!("Hello rCore-Tutorial.");
    println!("Hello, GuiYi.");
    //初始化各模块
    interrupt::init();
    memory::init();

    /*
    let remap = memory::mapping::memory_set::MemorySet::new_kernel().unwrap();
    println!("try to activate remap...");
    remap.activate();
    println!("remap is activated.");
    */

    {
        let kernel_process = Process::new_kernel().unwrap();
        let mut processor = PROCESSOR.get();
        for message in 0..8 {
            processor.add_thread(create_kernel_thread(
                kernel_process.clone(),
                sample_process as usize,
                Some(&[message]),
            ));
        }
    }

    unsafe {
        PROCESSOR.unsafe_get().run();
    }
}

fn start_kernel_thread(entry_point: usize, arguments: Option<&[usize]>) {
    use process::*;
    let process = Process::new_kernel().unwrap();
    let thread = Thread::new(process, entry_point, arguments).unwrap();
    PROCESSOR.get().add_thread(thread);
}

/// 创建一个内核进程
pub fn create_kernel_thread(
    process: Arc<RwLock<Process>>,
    entry_point: usize,
    arguments: Option<&[usize]>,
) -> Arc<Thread> {
    // 创建线程
    let thread = Thread::new(process, entry_point, arguments).unwrap();
    // 设置线程的返回地址为 kernel_thread_exit
    thread
        .as_ref()
        .inner()
        .context
        .as_mut()
        .unwrap()
        .set_ra(kernel_thread_exit as usize);

    thread
}

fn sample_process(message: usize) {
    for i in 0..1000000 {
        if i % 200000 == 0 {
            println!("thread {}", message);
        }
    }
}

/// 内核线程需要调用这个函数来退出
fn kernel_thread_exit() {
    use process::*;
    // 当前线程标记为结束
    PROCESSOR.get().current_thread().as_ref().inner().dead = true;
    // 制造一个中断来交给操作系统处理
    unsafe { llvm_asm!("ebreak" :::: "volatile") };
}
