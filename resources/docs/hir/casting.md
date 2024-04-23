# Casting

## Type suffixes

When using literals like strings and numbers, you might want to excplicitly type them (type suffix). In rust for example you can achieve it like this:

```rust
fn test() {
    return 0i32;
}
```

This will specify the integer to be of type `i32`

[WIP]

In citadel you can do the same thing using colon (`:`) syntax:

```txt
func @test() void {
    ret l{0}:i32
}
```

## Type casting

For casting between different types you can use the `cast` keyword. Similar to the arithmetic expressions you need to provide two arguments. The first for specifying the type and the second for specifying the expression that should be cast.

```txt
func @test() void {
    $x i32 = add l{10}, l{0} # Adding two numbers as an example
    $y f32 = %x:f32 # Type suffixes
    $z [char] = cast [char], %x # castomg the float to a string. Of course this is less than optimal casting and should probably not be done
}
```
