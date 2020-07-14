use riscv::register::sstatus::Sstatus;
//使用了rCore 中的库 riscv 封装的一些寄存器操作，需要在dependency中添加依赖
#[repr(C)]
pub struct Context {
    pub x: [usize; 32], //32个通用寄存器
    pub sstatus: Sstatus,
    pub sepec: usize,
}
