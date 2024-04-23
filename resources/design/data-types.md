# Composite data types

## Structs

Structs are a way to group data.

> If you come from an oop language
> like C#, Java, C++... you might
> be familiar with them since they
> basically are a low-lever version
> of a `class`

They are used when you want to order data,
pass multiple values (when returning for example),
create custom data types,
implement data structures etc...

You can define them like this.

```chir
struct @name {
    $x i32,
    $y i32,
}
```

A struct is defined similarlly to rust or C.

You start by specifying the name using the `@`
syntax (similar to functions, unions...).

After this you need to specify the fields of the
struct in a block. This is done similarly to function
args.

## Unions

Unions are quite similar to structs especially
when you only look at the syntax, there is one
key difference however: A union can only have
one value at a time.

> Rust has unions as well, however it makes more
> sense to use enums, as they have the same
> capabilities and are safer.

```chir
union @name {
    $test i32,
    $test1 i8,
}
```

A use case for a union would be, if you wanted to
have a general animal type that can represent
different types of animals.

So lets start by declaring a union that holds all
of the possible animal types.

```chir
union @AnimalData {
    $cat Cat,
    $dog Dog,
    # ... add more variants if needed
}
```

Of course we also need to define the `Cat` and `Dog` types.
For this we use structs. This also allows us to give them
individual data.

```chir
struct @Cat {
    # [i8; 100] is a string
    # with the size of 100
    $name [i8; 100],
    $height i8,
}

struct @Dog {
    $age i8,
    # in this case i8 is a boolean
    $hasTail i8,
}
```

We also need a way to easily figure out, which animal type
is stored in the union. For this we can use an `enum`.
In CHIR an enum is just an integer, so we can just use i8.
Now, finally we should wrap both the union and the "enum".
For this we can declare another struct.

```chir
$CAT_TYPE i8 = 0
$DOG_TYPE i8 = 1

struct @Animal {
    $data AnimalData,
    $type i8,
}
```
