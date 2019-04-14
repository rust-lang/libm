#![no_main]
#![no_std]

extern crate cortex_m;
extern crate cortex_m_rt as rt;

#[macro_use]
extern crate cortex_m_semihosting as semihosting;

extern crate panic_halt;

use core::{u32, usize};

use rt::entry;
use semihosting::debug::{exit, EXIT_FAILURE, EXIT_SUCCESS};
use semihosting::hio;

// Because this program will be executed within QEMU's user emulation mode we can use system calls!
extern "C" {
    fn __syscall1(nr: usize, a0: usize) -> usize;
    fn __syscall3(nr: usize, a0: usize, a1: usize, a2: usize) -> usize;
}

#[cfg(armv8m)]
#[entry]
fn main() -> ! {
    loop {
        asm::nop();
    }
}

#[cfg(not(armv8m))]
#[entry]
fn main() -> ! {
    run().unwrap_or_else(|_| {
        heprintln!("error").unwrap();
        exit(EXIT_FAILURE);
    });
    exit(EXIT_SUCCESS);
    loop {}
}

const EINTR: usize = 4;
const READ: usize = 3;

pub fn read(buffer: &mut [u8]) -> Result<usize, usize> {
    const FD: usize = 0;

    //let ec = unsafe { syscall!(READ, FD, buffer.as_mut_ptr() as usize, buffer.len()) } as isize;
    let ec = unsafe { __syscall3(READ, FD, buffer.as_mut_ptr() as usize, buffer.len()) } as isize;

    if ec < 0 {
        Err(-ec as usize)
    } else {
        Ok(ec as usize)
    }
}

/// Read the exact number of bytes required to fill `buf`.
pub fn read_exact(mut buf: &mut [u8]) -> Result<(), usize> {
    while !buf.is_empty() {
        match read(buf) {
            Ok(0) => break,
            Ok(n) => {
                let tmp = buf;
                buf = &mut tmp[n..];
            }
            Err(EINTR) => {}
            Err(e) => return Err(e),
        }
    }

    if !buf.is_empty() {
        Err(usize::MAX)
    } else {
        Ok(())
    }
}

#[no_mangle]
pub fn __errno() -> *mut i32 {
    static mut ERRNO: i32 = 0;
    unsafe { &mut ERRNO }
}
