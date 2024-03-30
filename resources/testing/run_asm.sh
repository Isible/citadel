nasm -f elf64 -o main.o main.asm
ld -s -o out/main main.o
rm main.o
./out/main