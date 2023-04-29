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

## Diffie-Hellman Key Exchange Example

Let's implement a Diffie-Hellman key exchange using elliptic curves.

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

## Let's hack Bob and Alice

We know that Alice will generate her shared key using her private key and multiplying it with the public generator point:

```rust
let alice_sk = g * alice_pk;
```

Hacking Alice means finding `alice_pk` and that is solving DLP. DLP is a _hard_ problem and there are many algorithms. Next I will present two of them:

### Brute force

Brute force is a naive approach that can solve this problem for small subgroups. As the order of the subgroup increases, chances are there is not enough computational power to solve a given problem.

```rust
// x*self = target
pub fn solve_dlp_brute_force(&self, target: ECPoint) -> Option<u64> {
    let mut xp = *self;
    let mut x = 1;
    let infinity = ECPoint::infinity(self.a, self.b);
    while xp != infinity {
        if xp == target {
            return Some(x);
        }
        x += 1;
        xp += *self;
    }
    None
}

let alice_pk = g.solve_dlp_brute_force(alice_sk);
```

### Baby step giant step

This algorithm can solve DLP in fewer steps:

$$ xP = Q $$
$$ (mj + i)P = Q $$
$$ iP = Q - mjP$$

First we choose $m = ceil(\sqrt{n})$

Then we calculate $iP$ for $i \in \{1, 2, ... m\}$

After that we iterate j and calculate $Q - mjP$ until we find a collision with the first list. When we find a collision it means we found $i$ and $j$ such that $iP = Q - mjP$ so we can calculate and return:

$$x = mj + i$$

```rust
pub fn solve_dlp_baby_step_giant_step(&self, target: ECPoint) -> Option<u64> {
    let m = (self.order() as f64).sqrt().ceil() as u64;
    let mut baby_steps = HashMap::new();
    let mut pi = *self;
    for i in 1..m {
        baby_steps.insert(pi, i);
        pi += *self;
    }

    let mp = m * *self;
    let mut jmp = ECPoint::infinity(self.a, self.b); // j*m*p not jump :P
    for j in 0..m {
        let q = target + (-jmp);

        if let Some(i) = baby_steps.get(&q) {
            return Some(m * j + i);
        }

        jmp += mp;
    }

    None
}

let alice_pk = g.solve_dlp_baby_step_giant_step(alice_sk);
```

## Comparing generator points

In the previous example we chose an arbitrary g = (13, 15) but we could have chose any other and there are advantages on choosing some over others.

Let's compare $g_1$ = (13, 15) and $g_2$ = (9, 2):

```rust
let mut gi = g1;
let mut order_1 = 1;
while gi != ECPoint::infinity(a, b) {
    order_1 += 1;
    gi += g1;
}
println!("Order of Generator Point 1: {}", order_1);

println!("Generator Point 2: {}", g2);

let mut gi = g2;
let mut order_2 = 1;
while gi != ECPoint::infinity(a, b) {
    order_2 += 1;
    gi += g2;
}

println!("Order of Generator Point 2: {}", order_2);

assert!(order_2 > order_1);
```

Output:
```
Generator Point 1: (13, 15)
Order of Generator Point 1: 13
Generator Point 2: (9, 2)
Order of Generator Point 2: 39
```

The order of $g_2$ is greater and $g_1$'s, this means $g_2$'s subgroup has more elements and makes guessing a private key harder. This algorithm's security depends on how hard the Discrete Logarithm Problem is to solve for the chosen g. The lower the order of the element the easier it is to solve DLP.
