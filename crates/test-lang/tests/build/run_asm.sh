nasm -f elf64 -o out.o codegen-test.asm
ld -s -o out out.o
rm out.o
./out