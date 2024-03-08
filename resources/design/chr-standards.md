# Citadel-Highlevel-IR Standards

## Variables

### Definition

Define a variable with `$` or `?` (`$` indicates a constant, `?` indicates a variable),

the name of the variable (myVar)

the visibility of the variable (lcl, see: [visibility mods](#access-mods))

the type of the var (see: [types](#types)) (i32)

and the value (integer literal l{...})

```text
$myVar lcl i32 = l{0}
```

### Referencing

Reference a variable or function with `%`

```text
%myVar
```

## Functions

...

### Access mods

- `lcl` make a value local/private
- `pub` make a value global/public

## Types

### Integer

- i1, - a boolean that can represent 0 or 1
- i8, - can represent a byte
- i6, - can represent a halfword
- i32, - can represent a word (a word is the unit of data the computer can process in a single operation)
- i64, - can represent a double word
- i128 - can represent a quad word
- i\<n> - a "customizable" integer

### Floats

- f16
- f32
- f64
- f128
- f\<n> - a "customizable" float
