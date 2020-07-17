//!实现用线段树对物理页面的分配
//!
//!一次只分配一个页,一次只回收一个页
use super::Allocator;
use alloc::{collections::vec_deque::VecDeque, vec::Vec};

struct SegTreeNode {
    l: usize,
    h: usize,
    position: usize, //当前单元区间，用其下限表示
    tags: [bool; 3], //标识区间是否已经占满 0:当前元区间;1:整个左边的区间段;2:右边区间段
}
pub struct SegTreeAllocator {
    heap: Vec<SegTreeNode>, //开一个满二叉树来构建线段树，每个结点代表一个单元区间，同时还能标识一个大区间
    real_size: usize,       //实际可以分配的单元区间最大下限
}

impl Allocator for SegTreeAllocator {
    fn new(size: usize) -> Self {
        //构造一个完全二叉搜索堆，避免对指针的使用
        //可用区间为[0, size+1]
        //每个结点代表一个单元区间，则原来有size+1个单元区间
        //现在构建满二叉树，令M为2^k-1，M>= size+1，对应的区间为[0, M]
        //将[size+1,M]中的所有单元区间初始化为已经占用
        let mut vec = Vec::<SegTreeNode>::new();
        let mut m = 1;
        let mut k: usize = 2;
        while m < size + 1 {
            k = k << 1;
            m = k - 1;
        }

        //用一个队列来建堆
        let mut queue = VecDeque::<SegTreeNode>::new();
        queue.push_back(SegTreeNode {
            l: 0,
            h: m,
            position: (0 + m) >> 1,
            tags: [false; 3],
        });
        while !queue.is_empty() {
            let node = queue.pop_front().unwrap();
            let l = node.l;
            let h = node.h;
            let mid = (l + h) >> 1;
            vec.push(node);
            if l < mid {
                queue.push_back(SegTreeNode {
                    l,
                    h: mid,
                    position: (l + mid) >> 1,
                    tags: [false; 3],
                });
            }
            if mid + 1 < h {
                queue.push_back(SegTreeNode {
                    l: mid + 1,
                    h,
                    position: (mid + 1 + h) >> 1,
                    tags: [false; 3],
                });
            }
        }

        //最后一层单元区间，左右的tag初始化为true
        for i in m / 2..m {
            vec[i].tags[1] = true;
            vec[i].tags[2] = true;
        }
        //对于添加的额外单元区间，current的tag初始化为true
        for i in 0..m {
            if vec[i].position > size {
                vec[i].tags[0] = true;
            }
        }
        //自底向上调整非叶子结点的左右孩子域的tag
        //序号从0开始，则左右孩子分别是i*2+1和i*2+2
        for i in (0..m / 2).rev() {
            vec[i].tags[1] =
                vec[i * 2 + 1].tags[0] && vec[i * 2 + 1].tags[1] && vec[i * 2 + 1].tags[2];
            vec[i].tags[2] =
                vec[i * 2 + 2].tags[0] && vec[i * 2 + 2].tags[1] && vec[i * 2 + 2].tags[2];
        }

        Self {
            heap: vec,
            real_size: size,
        }
    }

    /// 分配一个元素，返回被分配的单元的下标，无法分配则返回 `None`
    fn alloc(&mut self) -> Option<usize> {
        //按照先根遍历的顺序分配单个页面，即先看当前元区间，再左边区间，再右边区间
        let mut p = 0;
        while p < self.heap.len() && self.heap[p].tags[0] {
            if !self.heap[p].tags[1] {
                p = p * 2 + 1;
            } else {
                p = p * 2 + 2;
            }
        }
        if p < self.heap.len() {
            self.heap[p].tags[0] = true;
            //依次往上调整祖先结点的值
            let mut p1 = p;
            while p1 > 0 && self.heap[p1].tags[0] && self.heap[p1].tags[1] && self.heap[p1].tags[2]
            {
                let p2 = (p1 - 1) / 2; //父结点的索引
                if p1 == p2 * 2 + 1 {
                    //p1为左孩子
                    self.heap[p2].tags[1] = true;
                } else {
                    //p1为右孩子
                    self.heap[p2].tags[2] = true;
                }
                p1 = p2;
            }
            Some(self.heap[p].position)
        } else {
            None
        }
    }
    /// 回收一个元素
    fn dealloc(&mut self, index: usize) {
        if index > self.real_size {
            //println!("try to dealloc an invalid page index.");
            return;
        }
        let mut p = 0;
        while self.heap[p].position != index {
            if index < self.heap[p].position {
                //左边的区间必然没有满
                self.heap[p].tags[1] = false;
                p = p * 2 + 1;
            } else {
                //右边的区间必然没有满
                self.heap[p].tags[2] = false;
                p = p * 2 + 2;
            }
        }
        self.heap[p].tags[0] = false;
    }
}
