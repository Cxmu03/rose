global start

section .text
bits 32
start:
    ; Prints "Hello from rose os"
    mov dword [0xb8000], 0x0F650F48
    mov dword [0xb8004], 0x0F6C0F6C
    mov dword [0xb8008], 0x0F200F6F
    mov dword [0xb800C], 0x0F720F66
    mov dword [0xb8010], 0x0F6D0F6F
    mov dword [0xb8014], 0x0F720F20
    mov dword [0xb8018], 0x0F730F6F
    mov dword [0xb801C], 0x0F200F65
    mov dword [0xb8020], 0x0F730F6F
    hlt
