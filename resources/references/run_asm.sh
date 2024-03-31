nasm -f elf64 -o out.o stack.asm
ld -s -o out out.o
rm out.o
./out