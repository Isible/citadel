nasm -f elf64 -o out.o compiler-test.asm
ld -s -o out out.o
rm out.o
./out