global long_mode_start

extern kernel_main

section .text
bits 64
long_mode_start:
    call reset_segment_registers

    ; This is where the fun begins
    jmp kernel_main

reset_segment_registers:
    ; load 0 into all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ret