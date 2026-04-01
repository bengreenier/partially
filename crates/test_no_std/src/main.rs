#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { libc::abort() }
}

// Prebuilt `core` still references this symbol when linking a `no_std` binary.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

//////////////////////////////////////////////////////////////////////////////

use partially::Partial;

#[derive(Partial)]
struct Test {
    data: i32,
}

fn run_checks() {
    let mut base = Test { data: 1 };
    let partial = PartialTest { data: Some(2) };

    assert!(base.apply_some(partial));

    let mut base = PartialTest { data: None };
    let partial = PartialTest { data: Some(2) };

    assert!(base.apply_some(partial));
}

// Hosted `no_std`: libc's `crt` already provides `_start`; we only export `main`.
#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn main() -> i32 {
    run_checks();
    0
}

#[cfg(test)]
mod tests {
    use super::run_checks;

    #[test]
    fn derives_and_applies_in_test_mode() {
        run_checks();
    }
}
