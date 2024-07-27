# Variables

Variables are used for storing mutable data

## Definition

A variable consists of the following components:

```text
$myVar i32 = l{0:i32}
```

- `$` to indicate the declaration of a variable (similar to `let`, `var`, `const` in other languages)

- `myName` or any sequence of valid characters `a-z`, `A-Z`, `0-9`, `_`

- `i32` or any valid type. For builtin types see: [Builtin-Types](types#builtin-types), for declaring your own types, see: [Custom-Types](types#custom-types)

- and the value of the variable... [WIP]

## Referencing

Reference a variable or function with `%`

```text
%myVar
```
