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

       # Call the Rust entry point
       call    entry

       # We shouldn't return, but clean up anyway
       mov     %rbp, %rsp
       pop     %rbp
       hlt
