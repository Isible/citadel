# Intermediary-Representation-Introduction

## What is and Intermediary Representation?

The intermediary representation in a compiler is used to break down language source code into a simpler form that can more easily be optimized and translated into machine code. There are a few layers of intermediary representations. Citadel for example needs at least two layers, the high- and low-level representations. If optimization is toggled citadel will put more layers in-between these two.

## How does citadel handle IR

By default citadel uses 2 layers of intermediary representation.

### Citadel-highlevel-representation (chir)

This is the first layer of Intermediary representation that needs to be generated, in order for Citadel to perform optimizations and cleanup

Comparsion:

#### C

```C
// The main function
int main(void) {
    const int x = 100;
    char myString[] = "Hello World";
    return 0;
}
```

#### CIR

```chr
# The main function
func @main() i32 {
    # l{...} means that the value is a literal
    $x i8 = l{100:i8}
    $myString [i8; 11] = l{"Hello World":[i8; 11]}
    ret l{0:i32}
}
```

### Citadel-lowlevel-representation (clir)

This is the second mandatory layer. Citadel however will automatically compile to it from the first layer. It's syntax looks a lot like assembly tho a bit more readable and simplified. This is supposed to be the last layer. However, as of right now it is not fully designed yet and there are still a lot of problems with it so it might move up one layer. Anyways, here is the comparsion, this time using assembly (program is the same as the one on top)

[WIP]
