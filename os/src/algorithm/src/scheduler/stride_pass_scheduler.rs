use super::*;
use alloc::collections::LinkedList;
use core::cmp::Ordering;

type UNSIGNED = u64;
type SIGNED = i64;
const MAX_STRIDE: UNSIGNED = UNSIGNED::MAX;
const DEFAULT_PRIORITY: UNSIGNED = 2;
const DEFAULT_STRIDE: UNSIGNED = 0;

#[derive(Clone)]
struct ThreadNode<ThreadType: Clone + Eq> {
    thread: ThreadType,
    id: usize,
    stride: UNSIGNED,
    pass: UNSIGNED,
}

//将无符号的a和b相加，如果溢出，只保留后面低位
fn overflow_add(a: &mut UNSIGNED, b: UNSIGNED) {
    if b <= UNSIGNED::MAX - *a {
        *a += b;
    } else {
        *a = *a - (UNSIGNED::MAX - b + 1);
    }
}

//当x和y之差的绝对值小于SIGNED的最大绝对值的时候，可以比较大小
fn overflow_cmp(x: UNSIGNED, y: UNSIGNED) -> Ordering {
    if x == y {
        Ordering::Equal
    } else if x >= y {
        if ((x - y) as SIGNED) < 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    } else {
        if ((y - x) as SIGNED) < 0 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl<ThreadType: Clone + Eq> PartialOrd for ThreadNode<ThreadType> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(overflow_cmp(self.stride, other.stride))
    }
}

impl<ThreadType: Clone + Eq> Ord for ThreadNode<ThreadType> {
    fn cmp(&self, other: &Self) -> Ordering {
        overflow_cmp(self.stride, other.stride)
    }
}

impl<ThreadType: Clone + Eq> PartialEq for ThreadNode<ThreadType> {
    fn eq(&self, other: &Self) -> bool {
        self.stride == other.stride
    }
}

impl<ThreadType: Clone + Eq> Eq for ThreadNode<ThreadType> {}

pub struct StridePassScheduler<ThreadType: Clone + Eq> {
    pool: LinkedList<ThreadNode<ThreadType>>,
}

impl<ThreadType: Clone + Eq> Scheduler<ThreadType> for StridePassScheduler<ThreadType> {
    /// 优先级的类型
    type Priority = UNSIGNED;
    /// 向线程池中添加一个线程
    fn add_thread(&mut self, thread: ThreadType) {
        self.pool.push_back(ThreadNode {
            thread,
            id: self.pool.len(),
            stride: DEFAULT_STRIDE,
            pass: MAX_STRIDE / DEFAULT_PRIORITY,
        });
    }
    /// 获取下一个时间段应当执行的线程
    fn get_next(&mut self) -> Option<ThreadType> {
        if let Some(node) = self.pool.iter_mut().min() {
            overflow_add(&mut node.stride, node.pass);
            Some(node.thread.clone())
        } else {
            None
        }
    }
    /// 移除一个线程
    fn remove_thread(&mut self, thread: &ThreadType) {
        self.pool.drain_filter(|t| t.thread == *thread);
    }
    /// 设置线程的优先级
    fn set_priority(&mut self, thread: ThreadType, priority: Self::Priority) {
        assert!(
            priority > 1,
            "try to set a priority which is smaller than 1."
        );
        for node in self.pool.iter_mut() {
            if node.thread == thread {
                node.pass = MAX_STRIDE / priority;
                break;
            }
        }
    }

    fn get_count(&self) -> usize {
        self.pool.len()
    }
}

/// `Default` 创建一个空的调度器
impl<ThreadType: Clone + Eq> Default for StridePassScheduler<ThreadType> {
    fn default() -> Self {
        Self {
            pool: LinkedList::new(),
        }
    }
}
