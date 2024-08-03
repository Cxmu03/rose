global long_mode_start

section .text
bits 64
long_mode_start:
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

    hlt