# Opis Rust
Opis is a big integer, in little endian order, arithmetic library written in Rust.

## Usage

In Cargo.toml
```

[dependencies]
opis = "0.3.0"

```

In your rust file
```

use opis::Int;

```

## Features
- Representation of integers of any length.
- Addition.
- Get the binary representation of an integer.

## Functions

### From Str

```

let integer: Int = Int::from_str("6747", 10);

```

### Add

```

let sum: Int = int1.add(&int2);

```

### To Str

```

let binary_str: String = integer.to_str(2);

```

## Future Topics
- Subtraction
- Multiplication
- Division
- Remainder
- Full Base2, Base10 & Base16 conversion support

## Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2021-10-08

