pub fn baby_eve(alice_broadcasts: u64, bob_broadcasts: u64, public_base: u64) -> [u64; 3] {
    // Purpose:     Crack baby DH
    // Parameters:  alice's broadcast, bob's broadcast, and the public base
    // User-input:  None
    // Prints:      Nothing
    // Returns:     Should return an array of 3 unsigned ints:
    //              Alice's secret, Bob's secret, shared secret
    // Modifies:    Nothing
    // Calls:       ?
    // Tests:       unit_test_babydh.rs
    // Status:      Done, correct, but is it ideal?

    let alice_sec = (alice_broadcasts as f64).log(public_base as f64);

    let bob_sec = (bob_broadcasts as f64).log(public_base as f64);

    let shared_secret = (alice_broadcasts as u64).pow(bob_sec as u32);

    return [alice_sec as u64, bob_sec as u64, shared_secret as u64];
}

pub fn big_eve(
    alice_broadcasts: u64,
    bob_broadcasts: u64,
    public_base: u64,
    public_modulus: u64,
) -> [u64; 3] {
    // Purpose:      Crack real DH (albeit not with huge numbers)
    // Parameters:   Alice's broadcast, Bob's broadcast, the public base and modulus of DH.
    // User-input:   None
    // Prints:       Nothing
    // Returns:      Should return an array of 3 ints:
    //               Alice's secret, Bob's secret, shared secret
    // Modifies:     Nothing
    // Calls:        ?
    // Test:         ./unit_tests/unit_test_babydh.rs
    // Status:       TODO delete the 0 placeholders, and replace with real computations

    let mut alice_sec = 0;
    let mut bob_sec = 0;

    let mut alice_ps = 1;
    while alice_sec == 0 {
        let result = public_base.pow(alice_ps as u32).rem_euclid(public_modulus);
        if result == alice_broadcasts {
            alice_sec = result;
        } else {
            alice_ps = alice_ps + 1;
        }
    }
    alice_sec = alice_ps;
    let mut bob_ps = 1;
    while bob_sec == 0 {
        let result = public_base.pow(bob_ps as u32).rem_euclid(public_modulus);
        if result == bob_broadcasts {
            bob_sec = result;
        } else {
            bob_ps = bob_ps + 1;
        }
    }
    bob_sec = bob_ps;

    let shared_secret = alice_broadcasts
        .pow((bob_sec) as u32)
        .rem_euclid(public_modulus);

    [alice_sec as u64, bob_sec as u64, shared_secret as u64]
}
