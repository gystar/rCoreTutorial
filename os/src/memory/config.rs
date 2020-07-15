use super::address::*;
use lazy_static::*;

/// 操作系统动态分配内存所用的堆大小（8M）
pub const KERNEL_HEAP_SIZE: usize = 0x80_0000;

lazy_static! {
      /// 内核代码结束的地址，即可以用来分配的内存起始地址
    ///
    /// 因为 Rust 语言限制，我们只能将其作为一个运行时求值的 static 变量，而不能作为 const
    pub static ref KERNEL_END_ADDRESS : PhysicalAddress = PhysicalAddress(kernel_end  as usize);
}

extern "C" {
    /// 由 `linker.ld` 指定的内核代码结束位置
    ///
    /// 作为变量存在 [`static@KERNEL_END_ADDRESS`]
    pub fn kernel_end();
}
