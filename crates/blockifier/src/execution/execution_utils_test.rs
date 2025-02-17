use std::iter::zip;

use cairo_vm::felt::Felt252;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use pretty_assertions::assert_eq;
use starknet_api::hash::StarkFelt;

use crate::execution::execution_utils::{felt_to_stark_felt, stark_felt_to_felt};

fn starkfelt_to_felt_pairs() -> Vec<(StarkFelt, Felt252)> {
    // The STARK prime is 2 ^ 251 + 17 * 2 ^ 192 + 1.
    const STARK_PRIME_MINUS_ONE: &str =
        "0x800000000000011000000000000000000000000000000000000000000000000";

    let starkfelt_from_hex_error_message = "`StarkFelt` construction from hex has failed.";
    let starkfelts = vec![
        StarkFelt::from(0),
        StarkFelt::from(1),
        StarkFelt::from(1234),
        StarkFelt::try_from(STARK_PRIME_MINUS_ONE).expect(starkfelt_from_hex_error_message),
    ];
    let felts = vec![
        Felt252::zero(),
        Felt252::one(),
        Felt252::from(1234),
        // Note: the BigUint digits are ordered least significant digit first.
        // Prime - 1.
        Felt252::from(BigUint::new(vec![0, 0, 0, 0, 0, 0, 17, 134217728])),
    ];

    zip(starkfelts, felts).collect()
}

#[test]
fn test_stark_felt_to_felt() {
    for (stark_felt, equivalent_felt) in starkfelt_to_felt_pairs() {
        assert_eq!(stark_felt_to_felt(stark_felt), equivalent_felt);
    }
}

#[test]
fn test_felt_to_stark_felt() {
    for (equivalent_stark_felt, felt) in starkfelt_to_felt_pairs() {
        assert_eq!(felt_to_stark_felt(&felt), equivalent_stark_felt);
    }
}
