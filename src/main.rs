use rusty_diffie_hellman::{ec::ec_point::ECPoint, felt::felt::Felt};

fn diffie_hellman(g: ECPoint, alice_pk: u64, bob_pk: u64) {
    let alice_sk = g * alice_pk;
    let bob_sk = g * bob_pk;

    println!("Alice's Secret Key: {}", alice_sk);
    println!("Bob's Secret Key: {}", bob_sk);

    let alice_shared_secret = bob_sk * alice_pk;
    let bob_shared_secret = alice_sk * bob_pk;

    println!("Alice's Shared Secret: {}", alice_shared_secret);
    println!("Bob's Shared Secret: {}", bob_shared_secret);

    assert_eq!(alice_shared_secret, bob_shared_secret);
}

fn main() {
    println!("Diffie-Hellman Key Exchange");

    println!("Elliptic Curve: y^2 = x^3 + 6 (mod 43)");

    let modulus = 43;
    let a = Felt::new(0, modulus);
    let b = Felt::new(6, modulus);
    let x = Felt::new(13, modulus);
    let y = Felt::new(15, modulus);
    let g1 = ECPoint::new(x, y, a, b).unwrap();
    println!("Generator Point: {}", g1);

    let alice_pk = 7;
    let bob_pk = 11;

    diffie_hellman(g1, alice_pk, bob_pk);

    println!("=====================================");
    println!("Let's try again with a different generator point");

    let x = Felt::new(9, modulus);
    let y = Felt::new(2, modulus);
    let g2 = ECPoint::new(x, y, a, b).unwrap();

    let alice_pk = 8;
    let bob_pk = 25;

    diffie_hellman(g2, alice_pk, bob_pk);

    println!("=====================================");
    println!("Let's compare the order of the two generator points");

    println!("Generator Point 1: {}", g1);

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
}
