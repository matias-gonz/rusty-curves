# rusty-diffie-hellman
[![Tests](https://github.com/matias-gonz/rusty-diffie-hellman/actions/workflows/tests.yml/badge.svg)](https://github.com/matias-gonz/rusty-diffie-hellman/actions/workflows/tests.yml)

Diffie-Hellman key exchange implementation using Eliptic Curves.

## Primitives

### Field Element

The field element implements the basic operations: `Add`, `Sub`, `Mul`, `Div`, `Pow` and `Inverse`:

```rust
let f1 = Felt::new(5, 7);
let f2 = Felt::new(3, 7);
let f_add = f1 + f2;
let f_sub = f1 - f2;
let f_mul = f1 * f2;
let f_div = f1 / f2;
let f_inv = f1.inverse();
let f_pow = f.pow(5);
```

The inverse operation is done using the [Extended Euclidean Algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm).

The power calculation is optimized using bitwise operations. This calculates the power in constant time with a maximum of 64 iterations:

```rust
while exp > 0 {
    if exp % 2 == 1 {
        result = result * base;
    }
    exp >>= 1;
    base = base * base;
}
```

