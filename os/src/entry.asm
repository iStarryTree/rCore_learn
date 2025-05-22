    .section .text.entry
    .globl _start
_start:
    # li x1, 100  # Load 100 into x1
    la sp, boot_stack_top  # Set stack pointer to the top of the stack
    call rust_main  # Call the Rust main function

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
   .space 4096 * 16
   .globl boot_stack_top
boot_stack_top:
