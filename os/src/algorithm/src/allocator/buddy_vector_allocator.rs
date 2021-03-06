use super::super::unity::*;
use super::*;
//空闲表的长度
//则能分配的最大块为2^36=64G
//DynLinkedList为第一个节点静态分配，后续节点动态分配的链表
//伙伴算法初始化的时候，只需要一个结点
pub const MAX_POW: usize = 36 + 1;
pub struct BuddyAllocator {
    //所有能用的块，下标代表2的幂k
    //每个链表存储大小相同的块，用起始地址来表示
    blocks: [DynLinkedList<usize>; MAX_POW],
}

//min{k | 2^k >= n}
fn uper_bound_power(n: usize) -> (usize /*k*/, usize /*pow*/) {
    let mut k = 0;
    let mut v = 1;
    while v < n {
        v *= 2;
        k += 1;
    }
    (k, v)
}
impl VectorAllocator for BuddyAllocator {
    fn new(capacity: usize) -> Self {
        //将整个空间分成尽可能大的块放入链表
        let mut ins = Self {
            blocks: [DynLinkedList::new(); MAX_POW],
        };
        let mut size = capacity;
        let (mut max_k, min_pow) = uper_bound_power(capacity);
        if min_pow > capacity {
            max_k -= 1;
        }

        let mut addr = 0;
        while size > 0 {
            let (mut k, mut pow) = uper_bound_power(size);
            if pow > size {
                //分出尽可能的大的块
                k -= 1;
                pow >>= 1;
            }
            ins.blocks[k].push_back(addr);
            size -= pow;
            addr += pow;
        }
        ins
    }

    fn alloc(&mut self, size: usize, align: usize) -> Option<usize> {
        let (k, block_size) = uper_bound_power(size);
        for index in k..self.blocks.len() {
            if self.blocks[index].is_empty() {
                continue;
            }

            //检查每一个能满足大小条件的非空块链表，再检查对齐要求
            let mut list_item = 0;
            let mut p = self.blocks[index].get_first_addr();
            while p != 0 {
                let addr = self.blocks[index].next(&mut p);
                //找出第一个能划分出符合对齐要求的块
                //分小块的标号
                let mut off_set = 0;
                /*还是按照大块分两块然后继续分两块的方法，因此所有可能的地址为*/
                /*addr_new = addr+off_set*block_size; off_set -> [0, index-k]*/
                while off_set <= index - k
                    && (addr % align + (off_set * block_size) % align) % align != 0
                {
                    off_set += 1;
                }
                if off_set <= index - k {
                    let mut l = 0;
                    let mut h = index - k;
                    self.blocks[index].remove(list_item);
                    //当前要划分的块信息
                    let mut j = index;
                    let mut current_size = block_size << (index - k);
                    let mut addr_new = addr;
                    while l < h {
                        if off_set <= (l + h) / 2 {
                            //在左边
                            //右边一整块分出来放入空闲链表
                            self.blocks[j - 1].push_back(addr_new + current_size / 2);
                            h = (l + h) / 2;
                        } else {
                            //在右边
                            //左边一整块分出来放入空闲链表
                            self.blocks[j - 1].push_back(addr_new);
                            l = (l + h) / 2 + 1;
                            addr_new += current_size / 2;
                        }
                        current_size >>= 1;
                        j -= 1;
                    }
                    //最后剩下的即为所要分配出去的
                    return Some(addr_new);
                }
                list_item += 1;
            }
        }

        None
    }

    /// 回收指定空间（一定是之前分配的）
    fn dealloc(&mut self, start: usize, size: usize, align: usize) {
        //回收算法
        //伙伴的地址为:
        //start%(size*2)==0      => start+size
        //start%(size*2)==size   => start-size
        //找到伙伴的地址，合并为一个大块，并且迭代下去，直到伙伴不在空闲链表中
        let (k, mut block_size) = uper_bound_power(size);
        let mut m = k;
        let mut addr_new = start;
        while m < self.blocks.len() {
            let budd_addr;
            if addr_new % (block_size << 1) == 0 {
                budd_addr = addr_new + block_size;
            } else {
                budd_addr = addr_new - block_size;
            }
            if let Some(list_item) = self.blocks[m].find(budd_addr) {
                //伙伴块空闲则合并
                self.blocks[m].remove(list_item);
                if addr_new % (block_size << 1) != 0 {
                    addr_new = budd_addr;
                }
            } else {
                self.blocks[m].push_back(addr_new);
                break;
            }
            m += 1;
            block_size <<= 1;
        }
    }
}

impl Drop for BuddyAllocator {
    fn drop(&mut self) {
        for i in 0..MAX_POW {
            DynLinkedList::<usize>::destroy(&mut self.blocks[i]);
        }
    }
}
