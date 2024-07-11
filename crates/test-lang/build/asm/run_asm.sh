nasm -f elf64 -o main.o out.asm
ld -s -o main main.o
rm main.o
./main