#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn main() -> i32 {
    let mut buf = [0u8; 1024];

    loop {
        let msg = b"$: ";
        write(1, msg);

        let len = read(0, &mut buf);
        write(1, &buf[..len as usize]);

        if buf[0] == b'e' &&
           buf[1] == b'x' &&
           buf[2] == b'i' &&
           buf[3] == b't' {
            break;
        }
    }

    0
}

extern "C" {
    fn asm_write(fd: i32, buf: *const u8, len: usize) -> i32;
    fn asm_read(fd: i32, buf: *mut u8, len: usize) -> i32;
}

fn write(fd: i32, buf: &[u8]) -> i32 {
    let ret: i32;
    unsafe {
        ret = asm_write(fd, buf.as_ptr(), buf.len());
    }
    ret
}

fn read(fd: i32, buf: &mut [u8]) -> i32 {
    let ret: i32;
    unsafe {
        ret = asm_read(fd, buf.as_mut_ptr(), buf.len());
    }
    ret
}
