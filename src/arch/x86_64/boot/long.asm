global long_mode_start

extern kernel_main

section .text
bits 64
long_mode_start:
    call reset_segment_registers

    ; Prints "Hello from rose os"
    mov rax, 0x0F6C0F6C0F650F48
    mov qword [0xb8000], rax
    mov rax, 0x0F720F660F200F6F
    mov qword [0xb8008], rax
    mov rax, 0x0F720F200F6D0F6F
    mov qword [0xb8010], rax
    mov rax, 0x0F200F650F730F6F
    mov qword [0xb8018], rax
    mov dword [0xb8020], 0x0F730F6F

    jmp kernel_main

    hlt

reset_segment_registers:
    ; load 0 into all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    ret