//! 定义地址类型和地址常量
//!
//! 我们为虚拟地址和物理地址分别设立两种类型，利用编译器检查来防止混淆。
use super::config::*;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
/// 物理地址封装
pub struct PhysicalAddress(pub usize);

impl PhysicalAddress {
    //页内偏移
    fn page_offset(&self) -> usize {
        self.0 % PAGE_SIZE
    }
}

impl From<PhysicalPageNumber> for PhysicalAddress {
    fn from(page: PhysicalPageNumber) -> Self {
        Self(page.0 * PAGE_SIZE)
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
///物理页封装，连续4KB大小的空间
pub struct PhysicalPageNumber(pub usize);

impl PhysicalPageNumber {
    //将物理地址转化为页地址，向下取整
    pub fn floor(addr: PhysicalAddress) -> Self {
        Self(addr.0 / PAGE_SIZE)
    }
    //将物理地址转化为页地址，向上取整
    pub fn ceil(addr: PhysicalAddress) -> Self {
        Self(addr.0 / PAGE_SIZE + (addr.0 % PAGE_SIZE != 0) as usize)
    }
}

impl From<PhysicalAddress> for PhysicalPageNumber {
    fn from(addr: PhysicalAddress) -> Self {
        PhysicalPageNumber::floor(addr)
    }
}

//为PhysicalAddress和PhysicalPageNumber实现一些常见的+ - += -= 等操作
macro_rules! implement_usize_operations {
    ($type_name: ty) => {
        /// `+`
        impl core::ops::Add<usize> for $type_name {
            type Output = Self;
            fn add(self, other: usize) -> Self::Output {
                Self(self.0 + other)
            }
        }
        /// `+=`
        impl core::ops::AddAssign<usize> for $type_name {
            fn add_assign(&mut self, rhs: usize) {
                self.0 += rhs;
            }
        }
        /// `-`
        impl core::ops::Sub<usize> for $type_name {
            type Output = Self;
            fn sub(self, other: usize) -> Self::Output {
                Self(self.0 - other)
            }
        }
        /// `-`
        impl core::ops::Sub<$type_name> for $type_name {
            type Output = usize;
            fn sub(self, other: $type_name) -> Self::Output {
                self.0 - other.0
            }
        }
        /// `-=`
        impl core::ops::SubAssign<usize> for $type_name {
            fn sub_assign(&mut self, rhs: usize) {
                self.0 -= rhs;
            }
        }
        /// 和 usize 相互转换
        impl From<usize> for $type_name {
            fn from(value: usize) -> Self {
                Self(value)
            }
        }
        /// 和 usize 相互转换
        impl From<$type_name> for usize {
            fn from(value: $type_name) -> Self {
                value.0
            }
        }
        impl $type_name {
            /// 是否有效（0 为无效）
            pub fn valid(&self) -> bool {
                self.0 != 0
            }
        }
        /// {} 输出
        impl core::fmt::Display for $type_name {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                write!(f, "{}(0x{:x})", stringify!($type_name), self.0)
            }
        }
    };
}
implement_usize_operations! {PhysicalAddress}
implement_usize_operations! {PhysicalPageNumber}
