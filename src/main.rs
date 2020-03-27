#![no_std]
#![feature(
    intrinsics,
    core_intrinsics,
    asm,
    lang_items,
    start,
    box_syntax,
    panic_info_message,
    allocator_api,
    global_asm,
    linkage,
    c_variadic,
    box_into_raw_non_null,
    box_into_pin,
    maybe_uninit_ref,
    drain_filter,
    alloc_prelude,
    try_reserve
)]
#![cfg_attr(
    all(not(test), not(feature = "integration-test"), target_os = "none"),
    deny(warnings)
)]
#![allow(safe_packed_borrows)] // TODO(warnings)
#![allow(unused_attributes)] // TODO(warnings): getting unused attribute #[inline(always)] with rustc > 1.43.0 / abc3073c9 (and it's not clear why)

// TODO(cosmetics): Get rid of these three `extern crate` as we're in edition 2018:
extern crate alloc;
#[macro_use]
extern crate log;
#[macro_use]
extern crate klogger;

#[no_mangle]
pub fn main() {
    error!("error");
    warn!("warning");
    info!("info");
    debug!("debug");
    trace!("trace");
}
