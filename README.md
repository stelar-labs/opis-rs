# Opis

Opis is an arithmetic library for integer numbers.

## Author

Roy R. O. Okello

[Github] <https://github.com/0xR3y>
[Email] <mailto:0xR3y@protonmail.com>

## Usage

In your `Cargo.toml`:

```text
[dependencies]
opis = "4.0.0"
```

In your `module.rs`:

```text
use opis::{Bit, Int};
```

## Features

- Representation of integers of any magnitude.
- Bits are in two's complement and big endian order.
- Functions for arithmetic and conversions.

## API

### Int

- `Add: a + b`

- `AddAssign: a += b`

- `Sub: a - b`

- `SubAssign: a -= b`

- `Mul: a * b`

- `Div: a / b`

- `Rem: a % b`

- `Not: !a`

- `BitAnd: a & b`

- `BitOr: a | b`

- `BitXor: a ^ b`

- `Eq: a == b`

- `Ord: a > b`

- `Shift: a >> 1`

- `Shift Assign: a >>= 1`

- `Exponentiation: a.pow(&e)`

- `Modulo: a.modulo(&m)`

- `From Binary: Int::from_bin("b'1010101")`

- `To Binary: a.to_bin()`

- `From Decimal: Int::from_decimal("674755")`

- `To Decimal: a.to_dec()`

- `From Hexadecimal: Int::from_hex("0x00ABC012")`

- `To Hexadecimal: a.to_hex()`

- `From Bytes: Int::from_bytes(&bytes)`

- `To Bytes: a.to_bytes()`

- `To Extended Bytes: a.to_ext_bytes(32)`

- `To Extended Bits: a.to_ext_bits(256)`

- `Linear-feedback shift register: a.lfsr(1)`

## Contribution

Pull requests, bug reports and any kind of suggestions are welcome.

[Twitter] <https://twitter.com/StelarLabs>

2022-06-18
