'_entry: {
    call %main()
}
@main() i32 {
    call %print(l{"Hello World"})
    call %print(l{"I am a dino"})
    call %print(l{"and a cat"})
    call %print(l{"as well as a pig"})
    ret l{0}
}