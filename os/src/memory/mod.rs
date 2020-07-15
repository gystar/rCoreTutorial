pub mod address;
pub mod config;
pub mod frame;
pub mod heap;
pub mod range;

/// 一个缩写，模块中一些函数会使用
pub type MemoryResult<T> = Result<T, &'static str>;
pub use {address::*, config::*, frame::FRAME_ALLOCATOR, range::Range};

/// 初始化内存相关的子模块
///
/// - [`heap::init`]
pub fn init() {
    heap::init();
    println!("mod memory initialized.")
}
