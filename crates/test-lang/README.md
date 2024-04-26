# Test-lang [bin]

This is the test-lang crate. It provides a lexer, parser and compiler for a custom programming language called test-lang.
This language's syntax is inspired by other languages like Rust and C.
The main purpose of this language is to build an actual compiler using citadel and then use it to test and develop Citadel.
It also serves as an example implementation of a compiler.

Here is a hello world program in test-lang:

```test-lang
fn main(): void {
    puts("Hello, World!");
}
```

Here is a little bit more complex program that will add 10 and 20 and then return the squre of that sum as the exit code

```test-lang
fn main(): void {
    let sum: int = sum(2, 5);
    return square(sum);
}

fn sum(a: int, b: int): int {
    return a + b;
}

fn square(x: int): int {
    return x * x;
}
```
