SECTIONS {
    .stack (NOLOAD) : {
        . = ALIGN(16);
        __stack_start = .;
        . += 16384;
        __stack_end = .;
    }
    .abstack (NOLOAD) : {
        . = ALIGN(16);
        __ab_stack_start = .;
        . += 16384;
        __ab_stack_end = .;
    }
}
INSERT AFTER .bss;
