nasm -f elf64 -o out.o main.s
ld -s -o out out.o
rm out.o
./out