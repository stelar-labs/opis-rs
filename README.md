# Opis Rust
Opis is an arithmetic library for positive integers of any size written in Rust.

## Usage

In Cargo.toml
```

[dependencies]
opis = "0.6.0"

```

In your rust file
```

use opis::Int;

```

## Features
- Representation of integers of any length.
- Integer bits are stored in big endian bit order.
- There are three groups of functions: conversion, arithmetic & comparison
- Conversion functions support base 2, 10 & 16.
- Arithmetic functions are addition, subtration, multiplication, division & remainder.
- Comparision function checks between integers a and b if a is greater, lesser or equal to b.

## Conversion Functions

### From Str

```

let integer: Int = Int::from_str("6747", 10);

```

### To Str

```

let binary_str: String = integer.to_str(2);

```

## Arithmetic Functions

### Add

```

let sum: Int = int1.add(int2);

```

### Sub

```

let diff: Int = int1.sub(int2)?;

```

### Mul
```

let mul: Int = int1.mul(int2);

```

## Comparison Function

```

match &int1.cmp(&int2)[..] {
    "greater" => Println!("a is greater than b!"),
    "lesser" => Println!("a is lesser than b!"),
    "equal" => Println!("a is equal to b!"),
    _ => Println!("no match!"),
}

```

## Future Topics
- Division
- Remainder
- Full Base2, Base10 & Base16 conversion support

## Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2021-10-22

