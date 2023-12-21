#![no_std]
extern crate core;
#[inline(never)]
#[no_mangle]
#[doc = "Calls the Function \"print\" with the arguments \"ptr, len\"."]
pub unsafe extern "C" fn print(ptr: *const u8, len: usize) -> u64 {
    let mut _ret: u64;
    ::core::arch::asm!(
    "int 0x80",
    in("r8") 0x6,
    in("r9") ptr,
    in("r10") len,
    lateout("r8") _ret,
    options(nostack, nomem, raw)
    );
    return _ret & 0x6;
}
#[inline(never)]
#[no_mangle]
#[doc = "Calls the Function \"max_function\" with the arguments \"\"."]
pub unsafe extern "C" fn max_function() -> u64 {
    let mut _ret: u64;
    ::core::arch::asm!(
    "int 0x80",
    in("r8") 0xffff,
    lateout("r8") _ret,
    options(nostack, nomem, raw)
    );
    return _ret & 0xffff;
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { ::core::hint::unreachable_unchecked() }
}
