//! 页面置换算法

use super::*;
use crate::memory::{frame::FrameTracker, *};
use alloc::collections::VecDeque;
use alloc::vec::Vec;

/// 管理一个线程所映射的页面的置换操作
pub trait Swapper {
    /// 新建带有一个分配数量上限的置换器
    fn new(quota: usize) -> Self;

    /// 是否已达到上限
    fn full(&self) -> bool;

    /// 取出一组映射
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)>;

    /// 添加一组映射（不会在以达到分配上限时调用）
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, entry: *mut PageTableEntry);

    /// 只保留符合某种条件的条目（用于移除一段虚拟地址）
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool);
}

pub type SwapperImpl = ClockSwapper;

/// 页面置换算法基础实现：FIFO
pub struct FIFOSwapper {
    /// 记录映射和添加的顺序
    queue: VecDeque<(VirtualPageNumber, FrameTracker)>,
    /// 映射数量上限
    quota: usize,
}

impl Swapper for FIFOSwapper {
    fn new(quota: usize) -> Self {
        Self {
            queue: VecDeque::new(),
            quota,
        }
    }
    fn full(&self) -> bool {
        self.queue.len() == self.quota
    }
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        self.queue.pop_front()
    }
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, _entry: *mut PageTableEntry) {
        self.queue.push_back((vpn, frame));
    }
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.queue.retain(|(vpn, _)| predicate(vpn));
    }
}

pub struct ClockSwapper {
    queue: Vec<(
        VirtualPageNumber,
        FrameTracker,
        usize, /* *mut PageTableEntry */
    )>,
    /// 映射数量上限
    quota: usize,
}

//通过比较标志位(ACCESSED, DIRTY)来决定大小
//(0,0) < (0,1) < (1,0) < (1,1)
fn cmp_flags(a: usize, b: usize) -> bool {
    let entry_a = unsafe { *(a as *mut PageTableEntry) };
    let entry_b = unsafe { *(b as *mut PageTableEntry) };
    let (x1, x0) = entry_a.rw_falgs();
    let (y1, y0) = entry_b.rw_falgs();
    let x = (x1 as usize) * 2 + x0 as usize;
    let y = (y1 as usize) * 2 + y0 as usize;
    x < y
}

impl Swapper for ClockSwapper {
    fn new(quota: usize) -> Self {
        Self {
            queue: Vec::new(),
            quota,
        }
    }
    fn full(&self) -> bool {
        self.queue.len() == self.quota
    }
    fn pop(&mut self) -> Option<(VirtualPageNumber, FrameTracker)> {
        if self.queue.is_empty() {
            None
        } else {
            //从前往后找到最优先淘汰的页面
            let mut min_index = 0;
            for i in 1..self.queue.len() {
                if cmp_flags(self.queue[i].2, self.queue[min_index].2) {
                    min_index = i;
                }
            }
            let ret = self.queue.remove(min_index);
            Some((ret.0, ret.1))
        }
    }
    fn push(&mut self, vpn: VirtualPageNumber, frame: FrameTracker, _entry: *mut PageTableEntry) {
        assert!(self.queue.len() < self.quota);
        self.queue.push((vpn, frame, _entry as usize));
    }
    fn retain(&mut self, predicate: impl Fn(&VirtualPageNumber) -> bool) {
        self.queue.retain(|(vpn, _, _)| predicate(vpn));
    }
}
