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

extern "C" {
    fn asm_write(fd: i32, buf: *const u8, len: usize) -> i32;
}

fn write(fd: i32, buf: &[u8]) -> i32 {
    let ret: i32;
    unsafe {
        ret = asm_write(fd, buf.as_ptr(), buf.len());
    }
    ret
}
