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
    .heap (NOLOAD) : {
        . = ALIGN(8);
        __heap_start = .;
        . += 16384;
        __heap_end = .;
    }
}
INSERT AFTER .bss;
