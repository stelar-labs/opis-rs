# Opis

Opis is an arithmetic library for integer numbers.

## Author

Roy R. O. Okello

[Email](mailto:royokello@protonmail.com)

[Github](https://github.com/royokello)

[Twitter](https://twitter.com/RealOkello)

## Usage

### Cargo.toml

```text
[dependencies]
opis = "4.1.0"
```

### Module.rs

```text
use opis::{ Bit, Int };
```

## API

### Bit

```

add: a + b

and: a & b

eq: a == a

not: !a

or: a | b

xor: a ^ b

```

### Int

```

addition: a + b, a += b

and: a & b

base2: Int::from_bin("1010101"), a.to_bin()

base10: Int::from_dec("674755"), a.to_dec()

base16: Int::from_hex("00ABC012"), a.to_hex()

bytes: Int::from(&bytes), a.into()

extended bits: a.to_ext_bits(256)

extended bytes: a.to_ext_bytes(32)

comparision: a < b, a <= b, a > b, a >= b

division: a / b

equivalence: a == b

linear feedback shift register: a.lfsr(1)?

modulo: a.modulo(&m)

multiply: a * b, a *= b

negate: a.negate()

not: !a

or: a | b

exponentiation: a.pow(e)

remainder: a % b

shift left: a << 1, a <<= 1

shift right: a >> 1, a >>= 1

subtraction: a - b, a -= b

```

[Twitter](https://twitter.com/StelarLabs)

2022-10-18
