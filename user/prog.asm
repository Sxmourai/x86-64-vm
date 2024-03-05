; compile: nasm -O0 user/prog.asm -o user.o -f bin
; disasm: ndisasm user.o -b 64
BITS 64
mov ax, 0xdead
mov bx, 0xbeef
add ax, bx
jmp $