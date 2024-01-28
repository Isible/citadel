nasm -f elf64 -o main.o main.asm
ld -s -o main main.o
rm main.o
./main