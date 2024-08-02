global start

section .text
bits 32
start:
    mov esp, stack_bottom

    call verify_multiboot
    call verify_cpuid
    call verify_long_mode

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
    jne .no_multiboot
    ret
.no_multiboot:
    mov eax, error_not_multiboot_compliant
    jmp error

; Prints an error message to the VGA buffer
; Arguments:
;   eax: Error message
error:
    mov dword [0xb8000], 0x4f524f45             ; Prints Er
    mov dword [0xb8004], 0x4f3a4f52             ; Prints r:
    mov dword [0xb8008], 0x4f204f20             ; Prints two spaces
    xor ecx, ecx                                ; Set counter to 0
    xor ebx, ebx                                ; Set register for current character to 0
.print_loop:
    cmp byte [eax + ecx], 0                     ; Check if null terminator is found
    je .end                                     ; End of string reached
    mov bl, byte [eax + ecx]                    ; Move the current character to bl
    mov byte [(2 * ecx) + 0xb800c], bl          ; Move the character to the VGA buffer
    mov byte [(2 * ecx) + 0xb800c + 1], 0x4f    ; Set the background to red
    inc ecx                                     ; Increment counter
    jmp .print_loop
.end:
    hlt

; Check if CPUID is supported by attempting to flip the ID bit (bit 21)
; in the FLAGS register. If we can flip it, CPUID is available.
verify_cpuid:
    ; Copy FLAGS in to EAX via stack
    pushfd
    pop eax

    ; Copy to ECX as well for comparing later on
    mov ecx, eax

    ; Flip the ID bit
    xor eax, 1 << 21

    ; Copy EAX to FLAGS via the stack
    push eax
    popfd

    ; Copy FLAGS back to EAX (with the flipped bit if CPUID is supported)
    pushfd
    pop eax

    ; Restore FLAGS from the old version stored in ECX (i.e. flipping the
    ; ID bit back if it was ever flipped).
    push ecx
    popfd

    ; Compare EAX and ECX. If they are equal then that means the bit
    ; wasn't flipped, and CPUID isn't supported.
    cmp eax, ecx
    je .no_cpuid
    ret
.no_cpuid:
    mov eax, error_no_cpuid_present
    jmp error

verify_long_mode:
    mov eax, 0x80000000 ; Argument for CPUID
    cpuid
    cmp eax, 0x80000001 ; Needs to be atlease 0x80000001
    jb .no_long_mode    ; CPU is too old for long mode

    mov eax, 0x80000001 ; New Argument for CPUID
    cpuid
    test edx, (1 << 29) ; Test if long mode bit is set
    jz .no_long_mode    ; Long mode not available

    ret
.no_long_mode:
    mov eax, error_no_long_mode
    jmp error

error_not_multiboot_compliant:
    db "The bootloader used to load the image is not multiboot compliant", 0

error_no_cpuid_present:
    db "CPUID is not supported on your CPU", 0

error_no_long_mode:
    db "Your CPU is too old for long mode", 0

section .bss
stack_bottom:
    resb 64
stack_top:
