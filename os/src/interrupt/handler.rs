use super::context::Context;
use riscv::register::{scause::Scause, stvec};

global_asm!(include_str!("../interrupt.asm"));

/// 初始化中断处理
///
/// 在操作系统初始化时，会把中断入口 `__interrupt` 写入寄存器`stvec` 中，并且开启中断使能
pub fn init() {
    unsafe {
        extern "C" {
            /// `interrupt.asm` 中的中断入口
            fn __interrupt();
        }
        // 使用 Direct 模式，将中断入口设置为 `__interrupt`
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}

#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
    panic!("Interrupted:{:?}", scause.cause());
}
