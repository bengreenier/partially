#![allow(internal_features)]
#![feature(lang_items, start)]
#![no_std]

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    0
}

#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        libc::abort();
    }
}

//////////////////////////////////////////////////////////////////////////////

use partially::Partial;

#[derive(Partial)]
struct Test {
    data: i32,
}
