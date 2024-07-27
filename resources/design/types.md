# Types

## Builtin Types

### Integer

- i8, - can represent a byte (most commonly used for `char`)
- i16, - can represent a halfword
- i32, - can represent a word
- i64, - can represent a double word

a word is the unit of data the computer can process in a single operation <https://en.wikipedia.org/wiki/Word_(computer_architecture)>

### Floats

- f32
- f64

## Type suffixes

When using literals like strings and numbers, you need to excplicitly type them (with type suffixes). Most languages do not expect you to do this and will handle it for you. In C type suffix work like this:

```c
float test() {
    return 100f; // `f` specifies that this should be a float
}
```

This will specify the integer to be of type `i32`

In citadel you need do the same thing using colon (`:`) syntax:

(Suffixes for literals are a requirement)

```txt
func @test() i32 {
    ret l{0:i32}
}
```

## Type casting

For casting between different types you can use the `cast` keyword. Similar to the arithmetic expressions you need to provide two arguments. The first for specifying the type and the second for specifying the expression that should be cast.

```txt
func @test() void {
    $x i32 = add l{10:i32}, l{0:i32} # Adding two numbers as an example
    $y f32 = cast f32, %x # cast value `x` to a float
}
```
