use rusty_diffie_hellman::{ec::ec_point::ECPoint, felt::felt::Felt};

fn main() {
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
}
