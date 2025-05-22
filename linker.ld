ENTRY(_start)

SECTIONS {
    . = 0x100000;

    .text : ALIGN(0x1000) {
        PROVIDE(_text_start = .);
        *(.text._start)
        *(.text*)
        PROVIDE(_text_end = .);
    } :text

    .rodata : ALIGN(0x1000) {
        PROVIDE(_rodata_start = .);
        *(.rodata*)
        *(.rodata.*)
        PROVIDE(_rodata_end = .);
    } :rodata

    .eh_frame : {
        PROVIDE(_eh_frame_start = .);
        *(.eh_frame)
        *(.eh_frame.*)
        PROVIDE(_eh_frame_end = .);
    } :rodata

    .got : {
        PROVIDE(_got_start = .);
        *(.got)
        *(.got.*)
        PROVIDE(_got_end = .);
    } :data

    .data : ALIGN(0x1000) {
        PROVIDE(_data_start = .);
        *(.data*)
        *(.data.*)
        *(.sdata*)
        PROVIDE(_data_end = .);
    } :data

    /* Make sure the .got is initialized to zero when loaded into memory */
    .dynamic : {
        PROVIDE(_dynamic_start = .);
        *(.dynamic)
        PROVIDE(_dynamic_end = .);
    } :data

    .bss : ALIGN(0x1000) {
        PROVIDE(_bss_start = .);
        *(.bss*)
        *(.bss.*)
        *(.sbss*)
        *(COMMON)
        . = ALIGN(16);
        PROVIDE(_bss_end = .);
    } :data

    /* Discard debug sections */
    /DISCARD/ : {
        *(.comment)
        *(.debug_*)
        *(.note.*)
        *(.gcc_except_table)
    }
}

/* We need to initialize BSS to zero */
PROVIDE(__bss_size = _bss_end - _bss_start);

PHDRS
{
    text    PT_LOAD FLAGS(5);      /* 5 = rx */
    rodata  PT_LOAD FLAGS(4);      /* 4 = r */
    data    PT_LOAD FLAGS(6);      /* 6 = rw */
}
