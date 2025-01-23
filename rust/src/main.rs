#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
    let msg = b"Hello\n";
    write(1, msg);
    0
}

fn write(fd: i32, buf: &[u8]) -> i32 {
    let ret: i32;
    unsafe {
        core::arch::asm!(
            "mov x0, {:x}",          // stdout fd
            "mov x1, {}",          // buffer
            "mov x2, {}",          // length
            "mov x16, #4",         // write syscall
            "svc #0x80",          // syscall
            inout(reg) fd => ret,
            in(reg) buf.as_ptr(),
            in(reg) buf.len(),
        );
    }
    ret
}
