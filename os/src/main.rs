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
//! - `#![feature(naked_functions)]`
//!   允许使用 naked 函数，即编译器不在函数前后添加出入栈操作。
//!   这允许我们在函数中间内联汇编使用 `ret` 提前结束，而不会导致栈出现异常
#![feature(naked_functions)]

#[macro_use]
mod console;
mod drivers;
mod fs;
mod interrupt;
mod kernel;
mod memory;
mod panic;
mod process;
mod sbi;

extern crate alloc;
use memory::*;
use process::*;
use spin::RwLock;

use alloc::sync::Arc;

// 汇编编写的程序入口，具体见该文件
global_asm!(include_str!("entry.asm"));

/// Rust 的入口函数
///
/// 在 `_start` 为我们进行了一系列准备之后，这是第一个被调用的 Rust 函数
#[no_mangle] //禁用编译期间的名称重整（Name Mangling），保证生成命为_start的函数
pub extern "C" fn rust_main(_hart_id: usize, dtb_pa: PhysicalAddress) -> ! {
    println!("Hello rCore-Tutorial.");
    println!("Hello, GuiYi.");
    //初始化各模块
    memory::init();
    interrupt::init();
    drivers::init(dtb_pa);
    fs::init();
    println!(
        "kernel end:{:x}, dtb:{}",
        PhysicalAddress::from(*memory::KERNEL_END_ADDRESS).0,
        dtb_pa
    );

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
