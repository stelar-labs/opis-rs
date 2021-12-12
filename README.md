## Rust Opis
Opis is an arithmetic library for integer numbers written in Rust.

### Usage

In your `Cargo.toml`:

```

[dependencies]
opis = "1.1.0"

```

In your Rust file:

```

use opis::Int;

```

### Features
- Representation of integers of any length.
- Integer bits are stored in big endian order.
- The four groups of functions are conversion, arithmetic, comparison & bitwise.
- Conversion functions supports radix 2, 10 & 16 formatted strings.
- Arithmetic functions are addition, subtration, multiplication, division, remainder, modulo, exponentiation & modular inverse.
- Comparision functions for greater, less and equal.
- Four Bitwise functions are included: not, and, or, xor.

### API

#### Conversion Functions

`From Str`:

```

let integer: Int = Int::from_str("6747", 10)?;

```

`To Str`:

```

let binary_str: String = integer.to_str(2);

```

#### Arithmetic Functions

`Addition`

```

let s = int_a.add(&int2_b);

```

`Subtraction`

```

let d = int_a.sub(&int_b).unwrap();

```

`Multiplication`

```

let p = int_a.mul(&int_b);

```

`Division`

```

let q = int_a.div(&int_b).unwrap();

```

`Remainder`

```

let r = int_a.rem(&int_b).unwrap();

```

`Modulo`

```

let m = int_a.modulo(&int_b).unwrap();

```

`Exponentiation`

```

let p = int_a.pow(&int_e);

```

`Modular Inverse`

```

let mi = int_a.mod_inv(&int_m);

```

#### Comparison Functions

```

let ans_1: bool = int_a.is_greater(&int_b);

let ans_2: bool = int_a.is_less(&int_b);

let ans_3: bool = int_a.is_equal(&int_b);

```

#### Bitwise Functions

`NOT`

```

let n = int_x.not();

```

`AND`

```

let a = int_x.and(&int_y);

```

`OR`

```

let o = int_x.or(&int_y);

```

`XOR`

```

let x = int_x.xor(&int_y);

```

### Future
- Benchmarking Performance

### Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2021-12-12
