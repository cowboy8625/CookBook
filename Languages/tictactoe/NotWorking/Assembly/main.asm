format ELF64 executable 3
segment readable executable

X = 88
O = 79
READ_SYS = 0
WRITE_SYS = 1
EXIT_SYS = 60


entry main
print:
    mov rax, WRITE_SYS
    mov rdi, 1
    mov rsi, grid
    mov rdx, grid_len
    syscall
    ret

update:
    ; rdi/ dil index value into grid
    ; rsi/ sil char value
    xor rdi, rdi
    xor rsi, rsi
    mov dil, byte [input]
    sub dil, 49
    ; mov sil, X

    mov dl, [slots+edi]
    mov [grid+edx], sil
    ret

clear:
    mov rax, WRITE_SYS
    mov rdi, 1
    mov rsi, clear_str
    mov rdx, clear_str_len
    syscall
    ret

user_input:
    ; User input
    mov rax, READ_SYS
    xor rdi, rdi
    mov rsi, input
    mov rdx, input_len  ;5 bytes (numeric, 1 for sign) of that information
    syscall
    ret

main:
    push  1
game_loop:
    call  clear
    call  print
    call  user_input
    cmp   [input],    113
    je    .exit


    pop   rdi
    cmp   rdi,        0
    jz    .X
    jmp   .O
.X:
    push  1
    mov   sil,        X
    jmp   .E
.O:
    push  0
    mov   sil,        O
.E:


    call update
    jmp game_loop

.exit:
    mov rax, EXIT_SYS
    mov rdi, 0
    syscall

segment readable writable

input rb 10
input_len = $-input

slots db 0,2,4,12,14,16,24,26,28
slots_len = $-slots

grid db " | | ", 10, "-----", 10, " | | ", 10, "-----", 10, " | | ", 10
grid_len = $-grid

clear_str db 27, 91, 50, 74, 27, 91, 49, 59, 49, 72
clear_str_len = $-clear_str
