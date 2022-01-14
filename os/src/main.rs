#![no_std]
#![no_main]

// use std::prelude::*;

use core::arch::global_asm;
// 通过 include_str! 宏将同目录下的汇编代码 entry.asm 转化为字符串并通过 global_asm! 宏嵌入到代码中
global_asm!(include_str!("../prepare/entry.asm"));

#[no_mangle]
pub fn _rust_main() -> ! {
    std::println!("Hello, world!");
    panic!("Shutdown machine!");
}