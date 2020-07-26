use super::*;
use core::mem::size_of;
use core::slice::from_raw_parts_mut;
/// 将字符写入指定的文件
pub(super) fn sys_get_tid(fd: usize, buffer: *mut u8, size: usize) -> SyscallResult {
    println!("sys_get_tid!");
    let tid: isize = PROCESSOR.lock().current_thread().id;
    let buffer = unsafe { from_raw_parts_mut(buffer, size) };
    for (i, v) in (&tid.to_be_bytes()).iter().enumerate() {
        buffer[i] = *v;
    }
    SyscallResult::Proceed(8)
}
