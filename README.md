# rusty-curves
[![Tests](https://github.com/matias-gonz/rusty-curves/actions/workflows/tests.yml/badge.svg)](https://github.com/matias-gonz/rusty-curves/actions/workflows/tests.yml)

Elliptic curve primitives implemented using bare rust and math

## Primitives

### Field Element

A Felt(field element) constructor takes two parameters: `value` and `modulus`.
```rust
fn new(value: u64, modulus: u64)
```

The field element implements the basic operations: `Add`, `Sub`, `Mul`, `Div`, `Neg`, `Pow` and `Inverse`:

```rust
let f1 = Felt::new(5, 7);
let f2 = Felt::new(3, 7);

let f_add = f1 + f2;
let f_sub = f1 - f2;
let f_mul = f1 * f2;
let f_div = f1 / f2;
let f_neg = -f1;
let f_inv = f1.inverse();
let f_pow = f.pow(5);
```

The multiplicative inverse operation is done using the [Extended Euclidean Algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm). 

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

Felts can only operate with other Felts which have the same modulus, otherwise the operation will panic.

### Elliptic curve

Elliptic curve points support addition and multiplication. This is an example with the curve: $y^2 =x^3-3x-3$ with $p=1021$

```rust
let modulus = 1021;
let a = -Felt::new(3, modulus);
let b = -Felt::new(3, modulus);
let x = Felt::new(379, modulus);
let y = Felt::new(1011, modulus);
let p = ECPoint::new(x, y, a, b).unwrap();

let k = 655;
let kp = k * p;

println!("{}", kp);

// Outputs: (388, 60)
```

We can also calculate the number of points in that curve:
```rust
let modulus = 1021;
let a = -Felt::new(3, modulus);
let b = -Felt::new(3, modulus);
let n = ECPoint::get_all_points(a,b).len();

println!("{}", n);

//Outputs 1039
```

## Example

Elliptic Curve: $y^2 = x^3 + 6$ (mod 43)
```rust
println!("Diffie-Hellman Key Exchange");
println!("Elliptic Curve: y^2 = x^3 + 6 (mod 43)");

let modulus = 43;
let a = Felt::new(0, modulus);
let b = Felt::new(6, modulus);
let x = Felt::new(13, modulus);
let y = Felt::new(15, modulus);
let g = ECPoint::new(x, y, a, b).unwrap();
println!("Generator Point: {}", g);

let alice_pk = 7;
let bob_pk = 11;

let alice_sk = g * alice_pk;
let bob_sk = g * bob_pk;

println!("Alice's Secret Key: {}", alice_sk);
println!("Bob's Secret Key: {}", bob_sk);

let alice_shared_secret = bob_sk * alice_pk;
let bob_shared_secret = alice_sk * bob_pk;

println!("Alice's Shared Secret: {}", alice_shared_secret);
println!("Bob's Shared Secret: {}", bob_shared_secret);

assert_eq!(alice_shared_secret, bob_shared_secret);
```

Output:
```
Diffie-Hellman Key Exchange
Elliptic Curve: y^2 = x^3 + 6 (mod 43)
Generator Point: (13, 15)
Alice's Secret Key: (27, 9)
Bob's Secret Key: (33, 9)
Alice's Shared Secret: (13, 28)
Bob's Shared Secret: (13, 28)
```

We verify the shared secret is the same for both parties.
