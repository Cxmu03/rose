global start
extern long_mode_start

%define GDT_ENTRY_EXECUTABLE (1 << 43)
%define GDT_ENTRY_CODE_SEGMENT (1 << 44)
%define GDT_ENTRY_PRESENT (1 << 47)
%define GDT_ENTRY_LONG_SEGMENT (1 << 53)

%define PAGE_SIZE 4096

%define VGA_START 0xb8000

%define NEXT_VGA(counter_register, offset) ((2 * counter_register) + VGA_START + offset)

; Fills the next entry of the VGA buffer
; Only for use in the error method
; Parameter 1: color
; Parameter 2: ascii value
%macro SET_NEXT_VGA 2
    mov byte [NEXT_VGA(ecx, 0x0c)], %2
    mov byte [NEXT_VGA(ecx, 0x0c) + 1], %1
%endmacro

section .text
bits 32
start:
    mov esp, stack_bottom

    call verify_multiboot
    call verify_cpuid
    call verify_long_mode

    call setup_page_tables
    call enable_paging

    lgdt [gdt64.pointer]

    jmp gdt64.code:long_mode_start

enable_paging:
    ; Load the address of the p4 table into the cr3 register
    ; so that it can be used by the cpu
    mov eax, p4_table
    ; by settings the 8th bit in eax
    mov cr3, eax

    ; Enable the PEA flag in the cr4 register
    mov eax, cr4
    or eax, (1 << 5)
    mov cr4, eax

    ; Load IA32_EFER MSR and enable long mode
    ; by setting the 8th bit in eax
    mov ecx, 0xC0000080
    rdmsr
    or eax, (1 << 8)
    wrmsr

    ; Enable paging
    mov eax, cr0
    or eax, (1 << 31)
    mov cr0, eax

    ret

; Sets up identity paging for the first GiB of memory
setup_page_tables:
    ; Maps the p3 table as the first entry of the p4 table
    mov eax, p3_table
    or eax, 0b11        ; Page bits for present and writable
    mov [p4_table], eax

    ; Maps the p2 table as the first entry of the p3 table
    mov eax, p2_table
    or eax ,0b11        ; Page bits for present and writable
    mov [p3_table], eax

    xor ecx, ecx

.map_p2_table:
    mov eax, 0x200000             ; 2 MiB
    mul ecx                       ; ecx-th entry, eax is implied as operand and destination
    or eax, 0b10000011            ; Present + writable + huge
    mov [p2_table + ecx * 8], eax ; map ecx-th entry

    inc ecx
    cmp ecx, 512
    jne .map_p2_table

    ret

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
    mov dword [VGA_START], 0x4f524f45           ; Prints Er
    mov dword [VGA_START + 0x04], 0x4f3a4f52    ; Prints r:
    mov dword [VGA_START + 0x08], 0x4f204f20    ; Prints two spaces
    xor ecx, ecx                                ; Set counter to 0
    xor ebx, ebx                                ; Set register for current character to 0
; TODO: refactor with mul instruction
.print_loop:
    cmp byte [eax + ecx], 0                     ; Check if null terminator is found
    je .end                                     ; End of string reached
    mov bl, byte [eax + ecx]                    ; Move the current character to bl
    SET_NEXT_VGA 0x4f, bl
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


section .bss
align PAGE_SIZE
p4_table:
    resb PAGE_SIZE
p3_table:
    resb PAGE_SIZE
p2_table:
    resb PAGE_SIZE
stack_bottom:
    resb 64
stack_top:

section .rodata
error_not_multiboot_compliant:
    db "The bootloader used to load the image is not multiboot compliant", 0

error_no_cpuid_present:
    db "CPUID is not supported on your CPU", 0

error_no_long_mode:
    db "Your CPU is too old for long mode", 0

gdt64:
    dq 0                                             ; Null entry
.code: equ $ - gdt64                                 ; Label as offset into gdt
    ; Kernel code segment
    dq GDT_ENTRY_EXECUTABLE | \
       GDT_ENTRY_CODE_SEGMENT | \
       GDT_ENTRY_PRESENT | \
       GDT_ENTRY_LONG_SEGMENT
.pointer:
    dw $ - gdt64 - 1                                 ; Size of gdt
    dq gdt64                                         ; Address of gdt
