# Opis Rust
Opis is a big integer, in little endian order, arithmetic library written in Rust.

## Usage

In Cargo.toml
```

[dependencies]
opis = "0.1.0"

```

In your rust file
```

use opis::Int;

```

## Features
- Representation of integers of any length.
- Addition

## Functions

### New

```

let int1: Int = Int::new();

```

### From

```

let int2: Int = Int::from("6747");

```

### Add

```

let sum: Int = int1.add(&int2);

```

## Future Topics
- Multiplication

## Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2021-09-27

