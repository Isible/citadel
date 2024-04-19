nasm -f elf64 -o main.o codegen-test.asm
ld -s -o main main.o
rm main.o
./main