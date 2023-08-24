# Opis

Opis is a library for rational number and matrix arithmetic.

## Author

- Roy R. O. Okello: [Email](mailto:royokello@protonmail.com) & [GitHub](https://github.com/royokello)

## Features

- Arbitrary Precision Integer Representation and Arithmetic
- Fractions Arithmetic
- Matrix Arithmetic & Linear Regression

## Usage

### Import

```text
use opis::{ Bit, Integer, Fraction, Matrix };
```

### Bit

Add `a + b`

And `a & b`

Eq `a == a`

Not `!a`

Or `a | b`

Xor `a ^ b`

### Integer

pub struct Integer(pub Vec<Bit>)

- Hash
- Display
- Debug (:?, :#, :x, :b)

Addition `a + b, a += b`

And `a & b`

Base2 `Integer::from_bin("1010101"), a.to_bin()`

Base10 `Integer::from_dec("674755"), a.to_dec()`

Base16 `Integer::from_hex("00ABC012"), a.to_hex()`

Bytes `Integer::from(&bytes), a.into()`

Comparision `a < b, a <= b, a > b, a >= b`

Division `a / b?`

Equality `a == b`

Extended Euclidean Algorithm `a.ext_gcd(&b)`

Extended Bits `a.to_ext_bits(256)`

Extended Bytes `a.to_ext_bytes(32)`

Linear Feedback Shift Register `a.lfsr(1)?`

Modulo: `a.modulo(&m)`

Modular Exponentiation: `base.mod_pow(&exponent, &modulus)`

Multiply `a * b, a *= b`

Negate `a.negate()`

Not `!a`

Or `a | b`

Exponentiation `a.pow(e)?`

Remainder: `a % b?`

Shifts `a << 1, a <<= 1, a >> 1, a >>= 1`

#### String Conversion

`fn try_from(value: &str) -> Result<Self, Box<dyn Error>>`

- Binary (b'11, b'00, b'01)
- Decimal (-1, 0, 1)
- Hexadecimal (0xFF, 0x00, 0x01)

Subtraction `a - b, a -= b`

#### Type Conversion

- From: &[Bits], &[u8], u8, u16, u32, u64, u128, usize
- Into: Vec<u8>

#### String Conversion

Support: Binary, Decimal, Hexadecimal, Floating Literal

### Fraction

pub struct Fraction(pub Integer, pub Integer)

- Clone
- Hash
- Display
- Debug (:?, :#, :x, :b)

#### Addition

(a,b) + (c,d) = (ad + bc, bd)

- Add (+)
- AddAssign (+=)

#### Subtraction

(a,b) - (c,d) = (ad - bc, bd)

- Sub (-)
- SubAssign (-=)

#### Multiplication

(a,b) * (c,d) = (ac, bd)

- Mul (*)
- MulAssign (*=)

#### Division

(a,b) / (c,d) = (ad, bc) with c != 0

- Div (/) -> `Result<Fraction, Box<dyn Error>>`

#### Comparison & Equality

#### Additive Inverse / Opposite

-(a,b) = (-a,b)

- `fraction.opposite() -> Fraction`

#### Multiplicative Inverse / Reciprocal

(a,b)^-1 = (b,a)

- `fraction.reciprocal() -> Result<Fraction, Box<dyn Error>>`

#### Reduce

- `pub fn reduce(&mut self)`
- `pub fn reduction(&self) -> Fraction`

#### String Conversion

Support:  Binary, Decimal, Hexadecimal, Floating Point Literal

#### Type Conversion

From: Integer, u8, u16, u32, u64, u128, usize

### Matrix

#### Addition
`fn add(self, b: Self) -> Result<Matrix<T>, Box<dyn Error>>`

#### Cofactors
`fn cofactors(&self, neg_one: T) -> Result<Matrix<T>, Box<dyn Error>>`
```
A = [ 3 -1 -2]  C = [ 3  1 -2]
    [ 3  1 -1]      [-3  1  1]
    [ 3  4  2]      [ 3 -4  2]
```

#### Determinant
`fn determinant(&self, neg_one: T) -> Result<T, Box<dyn std::error::Error>>`

#### Dimensions
`fn dimensions(&self) -> Result<(usize, usize), Box<dyn Error>>`

#### Equality
`fn eq(&self, b: &Self) -> bool`

#### Identity
`fn identity(size: usize, zero: T, one: T) -> Matrix<T>`

#### inverse
`fn inverse(&self, neg_one: T, zero: T, one: T) -> Result<Matrix<T>, Box<dyn Error>>`

#### Linear Regression
`fn linear_regression(&self, variables: &Matrix<T>, neg_one: T, zero: T, one: T) -> Result<Matrix<T>, Box<dyn Error>>`

#### Minors
`fn minors(&self, neg_one: T) -> Result<Matrix<T>, Box<dyn Error>>`

#### Multiplication
`fn mul(self, b: Self) -> Result<Matrix<T>, Box<dyn Error>>`
`fn mul(self, b: T) -> Matrix<T>`

#### Subtraction
`fn sub(self, b: Self) -> Result<Matrix<T>, Box<dyn Error>>`

#### Trace
```
A = [-1  2  7  0]   Tr(A) = (-1 + 5 + 7 + 0) = 11
    [ 3  5 -8  4]
    [ 1  2  7 -3]
    [ 4 -2  1  0]
```
`fn trace(&self) -> Result<T, Box<dyn Error>>`

#### Transpose
`fn transpose(&self) -> Result<Matrix<T>, Box<dyn Error>>`
```
A = [2 0]   A' = [2 1 4]
    [1 1]        [0 1 3]
    [4 3]
```

## License

MIT License

Copyright Stelar Labs

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

## Disclaimer

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
