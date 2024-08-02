global start

section .text
bits 32
start:
    mov esp, stack_bottom

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

; Verifies that was started by a multiboot compliant bootloader
verify_multiboot:
    cmp eax, 0x36d76289
    je .no_multiboot
    ret
.no_multiboot:
    mov eax, error_not_multiboot_compliant
    jmp error

; Prints an error message to the VGA buffer
; Arguments:
;   eax: Error message
error:
    mov dword [0xb8000], 0x4f524f45
    mov dword [0xb8004], 0x4f3a4f52
    mov dword [0xb8008], 0x4f204f20
    xor ecx, ecx ; Set counter to 0
    xor ebx, ebx
.print_loop:
    cmp byte [eax + ecx], 0 ; Check if null terminator is found
    je .end      ; Go to end
    mov bl, byte [eax + ecx]
    mov byte [(2 * ecx) + 0xb800c], bl
    mov byte [(2 * ecx) + 0xb800c + 1], 0x4f
    inc ecx
    jmp .print_loop
.end:
    hlt

error_not_multiboot_compliant:
    db "The bootloader used to load the image is not multiboot compliant", 0

section .bss
stack_bottom:
    resb 64
stack_top:
