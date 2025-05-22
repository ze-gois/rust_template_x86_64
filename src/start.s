   .text
   .align  4
   .globl  _start
   .hidden _start
   .section .text._start
   .type   _start,@function
   _start:
       # Store the original stack pointer
       mov     %rsp, %rdi

       # Always ensure 16-byte alignment
       and     $-16, %rsp

       # Create a standard stack frame
       push    %rbp
       mov     %rsp, %rbp
       sub     $16, %rsp        # Reserve some stack space

       # Initialize BSS section to zero
       # bss_start and bss_end are provided by the linker script
       mov     $_bss_start, %rax
       mov     $_bss_end, %rcx
       cmp     %rcx, %rax
       je      bss_init_done
bss_zero_loop:
       movq    $0, (%rax)
       add     $8, %rax
       cmp     %rcx, %rax
       jl      bss_zero_loop
bss_init_done:
       
       # Initialize any relocations if needed
       # This would typically be done by the dynamic loader
       # For our static binary, we don't need much here

       # Call the Rust entry point
       call    entry

       # We shouldn't return, but clean up anyway
       mov     %rbp, %rsp
       pop     %rbp
       hlt
