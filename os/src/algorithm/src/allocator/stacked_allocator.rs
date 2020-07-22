//! 提供栈结构实现的分配器 [`StackedAllocator`]

use super::Allocator;

/// 使用栈结构实现分配器
///
/// 在 `Vec` 末尾进行加入 / 删除。
/// 每个元素 tuple `(start, end)` 表示 [start, end) 区间为可用。
///每次只分配一个单位，回收一个单位
///栈中可能有多个这样的数对，表示可用空间
const MAX_PAGES: usize = 0x8000;

#[derive(Copy, Clone, Debug)]
struct Section(usize, usize);
pub struct StackedAllocator {
    list: [Section; MAX_PAGES],
    top: usize,
}

impl Allocator for StackedAllocator {
    fn init(&mut self, capacity: usize) {
        self.list[0] = Section(0, capacity);
        self.top = 1;
    }

    fn alloc(&mut self) -> Option<usize> {
        if self.top > 0 {
            let st = self.list[self.top - 1];
            self.top -= 1;
            if st.1 - st.0 > 1 {
                //self.list.push((start + 1, end));
                self.list[self.top] = Section(st.0 + 1, st.1);
                self.top += 1;
            }
            Some(st.0)
        } else {
            None
        }
    }

    fn dealloc(&mut self, index: usize) {
        //self.list.push((index, index + 1));
        self.list[self.top] = Section(index, index + 1);
        self.top += 1;
    }
}
