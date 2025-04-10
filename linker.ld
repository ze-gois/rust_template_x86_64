ENTRY(_start)

SECTIONS {
    . = 0x100000;

    .text : ALIGN(0x1000) {
        *(.text._start)
        *(.text*)
    }

    .rodata : ALIGN(0x1000) {
        *(.rodata*)
    }

    .data : ALIGN(0x1000) {
        *(.data*)
    }

    .bss : ALIGN(0x1000) {
        *(.bss*)
        *(COMMON)
    }
}

PHDRS
{
    text    PT_LOAD FLAGS(5);      /* 5 = rx */
    rodata  PT_LOAD FLAGS(4);      /* 4 = r */
    data    PT_LOAD FLAGS(6);      /* 6 = rw */
}
