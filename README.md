# Opis Rust
Opis is an arithmetic library for integer numbers written in Rust.

## Usage

In Cargo.toml

```

[dependencies]
opis = "1.0.0"

```

In your rust file

```

use opis::Int;

```

## Features
- Representation of integers of any length.
- Integer bits are stored in big endian order.
- There are three groups of functions: conversion, arithmetic & comparison
- Conversion functions support radix 2, 10 & 16.
- Arithmetic functions are addition, subtration, multiplication, division, remainder, modulo, exponentiation & modular inverse.
- Comparision functions are is_greater, is_less and is_equal.

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

let s = int_a.add(&int2_b);

```

#### Subtraction(sub)

```

let d = int_a.sub(&int_b).unwrap();

```

#### Multiplication(mul)
```

let p = int_a.mul(&int_b);

```

#### Division(div)

```

let q = int_a.div(&int_b).unwrap();

```

#### Remainder(rem)

```

let r = int_a.rem(&int_b).unwrap();

```

#### Modulo(modulo)

```

let m = int_a.modulo(&int_b).unwrap();

```

#### Exponentiation(pow)

```

let p = int_a.pow(&int_e);

```

#### Modular Inverse(mod_inv)

```

let mi = int_a.mod_inv(&int_m);

```

### Comparison Functions

```

let ans_1: bool = int_a.is_greater(&int_b);

let ans_2: bool = int_a.is_less(&int_b);

let ans_3: bool = int_a.is_equal(&int_b);

```

## Performance
N/A

## Future
- Benchmarking Performance

## Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2021-11-05
