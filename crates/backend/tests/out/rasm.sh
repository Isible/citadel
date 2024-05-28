# runs the outputted assembly

nasm -f elf64 -o out.o out.asm
ld -s -o out out.o
rm out.o
./out
echo "Exit code: "$?