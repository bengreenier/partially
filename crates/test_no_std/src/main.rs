#![allow(internal_features)]
#![feature(lang_items, start)]
#![no_std]

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

#[start]
fn start(_argc: isize, _argv: *const *const u8) -> isize {
    let mut base = Test { data: 1 };
    let partial = PartialTest { data: Some(2) };

    assert!(base.apply_some(partial));

    let mut base = PartialTest { data: None };
    let partial = PartialTest { data: Some(2) };

    assert!(base.apply_some(partial));

    0
}
