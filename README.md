# Opis Rust
Opis is an arithmetic library for positive integers of any size written in Rust.

## Usage

In Cargo.toml

```

[dependencies]
opis = "0.7.0"

```

In your rust file

```

use opis::Int;

```

## Features
- Representation of integers of any length.
- Integer bits are stored in big endian order.
- There are three groups of functions: conversion, arithmetic & comparison
- Conversion functions support base 2, 10 & 16.
- Arithmetic functions are addition, subtration, multiplication, division, remainder & exponentiation.
- Comparision function checks between integers a and b if a is greater, lesser or equal to b.

## API

### Conversion Functions

#### From Str

```

let integer: Int = Int::from_str("6747", 10)?;

```

#### To Str

```

let binary_str: String = integer.to_str(2);

```

### Arithmetic Functions

#### Addition(add)

```

let s = int1.add(&int2);

```

#### Subtraction(sub)

```

let d = int1.sub(&int2).unwrap();

```

#### Multiplication(mul)
```

let p = int1.mul(&int2);

```

#### Division(div)

```

let q = int1.div(&int2);

```

#### Remainder(rem)

```

let r = int1.rem(&int2);

```

#### Exponentiation(pow)

```

let p = int1.pow(&int2);

```

### Comparison Function

```

match &int1.cmp(&int2)[..] {
    "greater" => Println!("a is greater than b!"),
    "lesser" => Println!("a is lesser than b!"),
    "equal" => Println!("a is equal to b!"),
    _ => Println!("no match!"),
}

```

## Performance

## Future Topics
- Full Base2, Base10 & Base16 conversion support

## Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2021-10-29
