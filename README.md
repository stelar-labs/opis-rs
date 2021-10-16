# Opis Rust
Opis is an arithmetic library for natural numbers written in Rust.

## Usage

In Cargo.toml
```

[dependencies]
opis = "0.4.0"

```

In your rust file
```

use opis::Int;

```

## Features
- Representation of integers of any length.
- There are three groups of functions: string conversion, basic arithmetic & comparison
- String Conversion functions support base 2, 10 & 16.
- Basic Arithmetic functions are addition, subtration, multiplication, division & remainder.
- Comparision function checks between integers a and b if a is greater, lesser or equal to b.

## String Conversion Functions

### From Str

```

let integer: Int = Int::from_str("6747", 10);

```

### To Str

```

let binary_str: String = integer.to_str(2);

```

## Basic Arithmetic Functions

### Add

```

let sum: Int = int1.add(&int2);

```

## Future Topics
- Subtraction
- Multiplication
- Division
- Remainder
- Full Base2, Base10 & Base16 conversion support

## Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2021-10-15

