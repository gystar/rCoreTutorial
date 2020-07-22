//! 提供帧分配器 [`FRAME_ALLOCATOR`](FrameAllocator)
//!
//! 返回的 [`FrameTracker`] 类型代表一个帧，它在被 drop 时会自动将空间补回分配器中。
use super::*;
use crate::memory::*;
use algorithm::{Allocator, AllocatorImpl};
use lazy_static::*;
use spin::Mutex;
/*
lazy_static! {
    /// 帧分配器
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator<StackedAllocator>> = Mutex::new(
        FrameAllocator::new(
            Range::from(
                PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS))..PhysicalPageNumber::floor(MEMORY_END_ADDRESS)
        )));
}
*/

lazy_static! {
    /// 帧分配器
    //注意：会预留一些物理页给分配使用
    pub static ref FRAME_ALLOCATOR: Mutex<FrameAllocator> = Mutex::new(
        FrameAllocator::new(
            Range::from(
                PhysicalPageNumber::ceil(PhysicalAddress::from(*KERNEL_END_ADDRESS))+core::mem::size_of::<AllocatorImpl>()/PAGE_SIZE+1..PhysicalPageNumber::floor(MEMORY_END_ADDRESS)
        )));
}

/// 基于线段树的帧分配 / 回收
pub struct FrameAllocator {
    /// 可用区间的起始
    start_ppn: PhysicalPageNumber,
    /// 分配器放置在物理页面中,这是其起始位置
    allocator_addr: VirtualAddress,
}

fn get_allocator<T: Allocator>(ptr: VirtualAddress) -> &'static mut T {
    let allocator = ptr.deref();
    allocator
}

impl FrameAllocator {
    /// 创建对象
    pub fn new(range: impl Into<Range<PhysicalPageNumber>> + Copy) -> Self {
        //将内核之后的一段页面用来存放物理页分配信息
        let allocator_addr: VirtualAddress = VirtualPageNumber::ceil(*KERNEL_END_ADDRESS).into();
        get_allocator::<AllocatorImpl>(allocator_addr).init(range.into().len());
        FrameAllocator {
            start_ppn: range.into().start,
            allocator_addr,
        }
    }

    /// 分配帧，如果没有剩余则返回 `Err`
    pub fn alloc(&mut self) -> MemoryResult<FrameTracker> {
        get_allocator::<AllocatorImpl>(self.allocator_addr)
            .alloc()
            .ok_or("no available frame to allocate")
            .map(|offset| FrameTracker(self.start_ppn + offset))
    }

    /// 将被释放的帧添加到空闲列表的尾部
    ///
    /// 这个函数会在 [`FrameTracker`] 被 drop 时自动调用，不应在其他地方调用
    pub(super) fn dealloc(&mut self, frame: &FrameTracker) {
        get_allocator::<AllocatorImpl>(self.allocator_addr)
            .dealloc(frame.page_number() - self.start_ppn);
    }
}
