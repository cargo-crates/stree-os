#![no_std]
#![feature(panic_info_message)] // panic! 时，获取其中的信息并打印

#[macro_use]
pub mod console;
pub mod sbi;
pub mod lang_items;

pub mod prelude;
