    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top #栈指针设置为先前分配的启动栈栈顶地址
    call _rust_main #调用rust入口函数

    .section .bss.stack #段名称（内存空间放置位置, 在链接脚本 linker.ld 中可以看到 .bss.stack 段最终会被汇集到 .bss 段中：）
    .globl boot_stack #标识栈低的位置
boot_stack:
    .space 4096 * 16 #64kb的空间
    .globl boot_stack_top #标识栈顶的位置
boot_stack_top: