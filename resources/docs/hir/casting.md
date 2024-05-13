# Casting

## Type suffixes

When using literals like strings and numbers, you need to excplicitly type them (with type suffixes). In rust for example you can achieve it like this:

```rust
fn test() {
    return 0i32;
}
```

This will specify the integer to be of type `i32`

In citadel you can do the same thing using colon (`:`) syntax:

```txt
func @test() void {
    ret l{0:i32}
}
```

## Type casting

For casting between different types you can use the `cast` keyword. Similar to the arithmetic expressions you need to provide two arguments. The first for specifying the type and the second for specifying the expression that should be cast.

```txt
func @test() void {
    $x i32 = add l{10:i32}, l{0:i32} # Adding two numbers as an example
    $y f32 = %x # Type suffixes
    $z [i8; 10] = cast [i8; 10], %x # casting the float to a string. Of course this is less than optimal casting and should probably not be done
}
```
