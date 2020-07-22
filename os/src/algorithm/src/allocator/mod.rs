//! 负责分配 / 回收的数据结构

mod bitmap_vector_allocator;
mod buddy_vector_allocator;
mod stacked_allocator;

///每次只分配一个单位，回收一个单位
//注意实现的时候，不能使用动态内存分配，需要使用固定数组，数组的长度为MAX_PAGES，即最大可能的页数量
pub trait Allocator {
    /// 给定容量，初始化分配器
    fn init(&mut self, capacity: usize);
    /// 分配一个元素，返回被分配的单元的下标，无法分配则返回 `None`
    fn alloc(&mut self) -> Option<usize>;
    /// 回收一个元素
    fn dealloc(&mut self, index: usize);
}

/// 向量分配器：固定容量，每次分配 / 回收一个带有对齐要求的连续向量
///
/// 参数和返回值中的 usize 表示第 n 个字节，不需要考虑起始地址
pub trait VectorAllocator {
    /// 给定容量，创建分配器
    fn new(capacity: usize) -> Self;
    /// 分配指定长度的空间，无法分配则返回 `None`
    fn alloc(&mut self, size: usize, align: usize) -> Option<usize>;
    /// 回收指定空间（一定是之前分配的）
    fn dealloc(&mut self, start: usize, size: usize, align: usize);
}

pub use bitmap_vector_allocator::BitmapVectorAllocator;
pub use buddy_vector_allocator::BuddyAllocator;
pub use stacked_allocator::StackedAllocator;

/// 默认使用的分配器
//单个物理页的分配器
pub type AllocatorImpl = StackedAllocator;
//
pub type VectorAllocatorImpl = BitmapVectorAllocator;
