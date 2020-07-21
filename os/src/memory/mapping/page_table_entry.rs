///riscv Sv59  三级页表的页表项的实现
use super::super::address::*;
use bit_field::*;
use bitflags::*;
#[derive(Copy, Clone, Default)]
pub struct PageTableEntry(usize);

/// Sv39 页表项中标志位的位置
const FLAG_RANGE: core::ops::Range<usize> = 0..8;
/// Sv39 页表项中物理页号的位置
const PAGE_NUMBER_RANGE: core::ops::Range<usize> = 10..54;

bitflags! {
    /// 页表项中的 8 个标志位
    #[derive(Default)]
    pub struct Flags: u8 {
        /// 有效位
        const VALID =       1 << 0;
        /// 可读位
        const READABLE =    1 << 1;
        /// 可写位
        const WRITABLE =    1 << 2;
        /// 可执行位
        const EXECUTABLE =  1 << 3;
        /// 用户位
        const USER =        1 << 4;
        /// 全局位，我们不会使用
        const GLOBAL =      1 << 5;
        /// 已使用位，用于替换算法
        const ACCESSED =    1 << 6;
        /// 已修改位，用于替换算法
        const DIRTY =       1 << 7;
    }
}

#[allow(unused)]
impl PageTableEntry {
    /// 将相应页号和标志写入一个页表项
    pub fn new(page_number: PhysicalPageNumber, flags: Flags) -> Self {
        Self(
            *0usize
                .set_bits(..8, flags.bits() as usize)
                .set_bits(10..54, page_number.into()),
        )
    }
    /// 清除
    pub fn clear(&mut self) {
        self.0 = 0;
    }
    /// 获取页号
    pub fn page_number(&self) -> PhysicalPageNumber {
        PhysicalPageNumber::from(self.0.get_bits(10..54))
    }
    /// 获取地址
    pub fn address(&self) -> PhysicalAddress {
        PhysicalAddress::from(self.page_number())
    }
    /// 获取标志位
    pub fn flags(&self) -> Flags {
        unsafe { Flags::from_bits_unchecked(self.0.get_bits(..8) as u8) }
    }
    /// 是否为空（可能非空也非 Valid）
    pub fn is_empty(&self) -> bool {
        self.0 == 0
    }
    /// 是否指向下一级（RWX 全为0）
    pub fn has_next_level(&self) -> bool {
        let flags = self.flags();
        !(flags.contains(Flags::READABLE)
            || flags.contains(Flags::WRITABLE)
            || flags.contains(Flags::EXECUTABLE))
    }
}

impl core::fmt::Debug for PageTableEntry {
    fn fmt(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        formatter
            .debug_struct("PageTableEntry")
            .field("value", &self.0)
            .field("page_number", &self.page_number())
            .field("flags", &self.flags())
            .finish()
    }
}

macro_rules! implement_flags {
    ($field: ident, $name: ident, $quote: literal) => {
        impl Flags {
            #[doc = "返回 `Flags::"]
            #[doc = $quote]
            #[doc = "` 或 `Flags::empty()`"]
            pub fn $name(value: bool) -> Flags {
                if value {
                    Flags::$field
                } else {
                    Flags::empty()
                }
            }
        }
    };
}

implement_flags! {USER, user, "USER"}
implement_flags! {READABLE, readable, "READABLE"}
implement_flags! {WRITABLE, writable, "WRITABLE"}
implement_flags! {EXECUTABLE, executable, "EXECUTABLE"}
