//可以用栈上的空间初始化，同时可以动态增长的双向链表
//第一个结点预留空间，后面的结点动态分配
use alloc::alloc::{alloc, dealloc, Layout};
use core::fmt::{Debug, Display};

//使用之后必须手动调用一次destroy进行内存释放,不管是否使用new来创建
#[derive(Debug, Copy, Clone, Default)]
pub struct DynLinkedList<T: Default + Copy + Debug + Eq + Display> {
    first: DynNode<T>,
    tail: usize, //尾结点的裸指针
    len: usize,
}

#[derive(Debug, Copy, Clone, Default)]
pub struct DynNode<T: Default + Copy + Debug + Eq + Display> {
    //数据
    value: T,
    //指向前一个结点的裸指针
    pre: usize,
    //指向下一个结点的裸指针
    next: usize,
}

impl<'a, T: Default + Copy + Debug + Eq + Display> DynNode<T> {
    //由结点的裸指针得到DynNode对象
    pub fn deref(addr: usize) -> &'a mut Self {
        unsafe { &mut *(addr as *mut Self) }
    }
    //获得node的实际地址
    pub fn get_addr(&self) -> usize {
        let ptr: *const Self = self as *const Self;
        ptr as usize
    }
    //在堆上分配一个结点
    pub fn new(value: &T, pre: usize, next: usize) -> &mut Self {
        unsafe {
            let layout = Layout::new::<Self>();
            let ptr = alloc(layout);
            let ret = &mut *(ptr as *mut Self);
            ret.value = *value;
            ret.pre = pre;
            ret.next = next;
            ret
        }
    }
    //使用new创建出来的结点需要调用此函数释放空间
    //在栈上非new方式生成的结点不需要手动desdory
    pub fn desdory(node: &mut Self) {
        unsafe {
            let layout = Layout::new::<Self>();
            let ptr: *mut Self = node as *mut Self;
            dealloc(ptr as *mut u8, layout);
        }
    }
}

impl<T: Default + Copy + Debug + Eq + Display> DynLinkedList<T> {
    pub fn new() -> Self {
        let mut list = Self {
            first: DynNode {
                value: Default::default(),
                next: 0,
                pre: 0,
            },
            tail: 0,
            len: 0,
        };
        list
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn push_back(&mut self, value: T) {
        if self.len == 0 {
            self.first.value = value;
            self.tail = self.first.get_addr();
            self.len += 1;
        } else {
            let node = DynNode::new(&value, self.tail, 0);
            DynNode::<T>::deref(self.tail).next = node.get_addr();
            self.tail = node.get_addr();
            self.len += 1;
        }
    }
    pub fn push_front(&mut self, value: T) {
        if self.len == 0 {
            self.first.value = value;
            self.tail = self.first.get_addr();
            self.len += 1;
        } else {
            let node = DynNode::new(&self.first.value, self.first.get_addr(), self.first.next);
            if self.len == 1 {
                self.tail = node.get_addr();
            } else {
                DynNode::<T>::deref(self.first.next).pre = node.get_addr();
            }
            self.first.next = node.get_addr();
            self.first.value = value;
            self.len += 1;
        }
    }
    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }
        let addr = self.tail;
        let mut last = DynNode::<T>::deref(addr);
        let value = last.value;
        if self.len == 1 {
            self.tail = 0;
            self.first.next = 0;
        } else {
            self.tail = last.pre;
            DynNode::<T>::deref(last.pre).next = 0;
        }
        if self.len > 1 {
            //尾结点为动态分配的，需要释放
            DynNode::<T>::desdory(last);
        }

        self.len -= 1;

        Some(value)
    }
    pub fn pop_front(&mut self) -> Option<T> {
        //将后一个结点(如果有)释放，并且把数据复制到头结点
        if self.len == 0 {
            return None;
        }
        if self.len == 1 {
            self.first.next = 0;
            self.len = 0;
            self.tail = 0;
            Some(self.first.value)
        } else {
            let value = self.first.value;
            let next_node = DynNode::<T>::deref(self.first.next);
            self.first.next = next_node.next;
            self.first.value = next_node.value;
            if next_node.next != 0 {
                DynNode::<T>::deref(next_node.next).pre = next_node.pre;
            }
            self.len -= 1;
            Some(value)
        }
    }
    pub fn remove(&mut self, at: usize) -> Option<T> {
        assert!(at < self.len, "DynLinkedList: remove out of range.");
        if at == 0 {
            self.pop_front()
        } else {
            let mut n = 1;
            let mut p = self.first.next;
            while n < at {
                p = DynNode::<T>::deref(p).next;
            }
            let mut node = DynNode::<T>::deref(p);
            DynNode::<T>::deref(node.pre).next = node.next;
            if node.next != 0 {
                DynNode::<T>::deref(node.next).pre = node.pre;
            }
            let value = node.value;
            DynNode::<T>::desdory(node);

            if p == self.tail {
                self.tail = node.pre;
            }
            self.len -= 1;
            Some(value)
        }
    }

    //在链表中查找，找到则返回下标，否则返回None
    pub fn find(&self, value: T) -> Option<usize> {
        if self.len == 0 {
            return None;
        }

        let mut p = self.first.get_addr();
        let mut index = 0;
        let mut node = DynNode::<T>::deref(p);
        while p != 0 && node.value != value {
            p = node.next;
            node = DynNode::<T>::deref(p);
            index += 1;
        }
        if p != 0 {
            Some(index)
        } else {
            None
        }
    }

    pub fn get_first_addr(&self) -> usize {
        if self.len == 0 {
            0
        } else {
            self.first.get_addr()
        }
    }
    //必须传入有效的地址，由get_first_addr初始化，每次都会更新p，不要手动给p赋值
    //p为0则到终点了
    pub fn next(&self, p: &mut usize) -> T {
        let node = DynNode::<T>::deref(*p);
        *p = node.next;
        node.value
    }

    pub fn print(&self) {
        if self.is_empty() {
            return;
        }

        let mut p = self.first.get_addr();
        while p != 0 {
            let node = DynNode::<T>::deref(p);
            //print!("{} ", node.value);
            p = node.next;
        }
    }

    pub fn print_rev(&self) {
        if self.is_empty() {
            return;
        }

        let mut p = self.tail;
        while p != 0 {
            let node = DynNode::<T>::deref(p);
            //print!("{} ", node.value);
            p = node.pre;
        }
    }
    //需要手动调用此函数来释放空间
    pub fn destroy(ins: &mut Self) {
        if ins.len < 2 {
            return;
        }
        let mut p = ins.first.next;
        while p != 0 {
            let mut node = DynNode::<T>::deref(p);
            let tmp = node.next;
            DynNode::<T>::desdory(&mut node);
            p = tmp;
        }
    }
}
