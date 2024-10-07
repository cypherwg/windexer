use ark_ed_on_bls12_381::Fq as F;
use ark_ff::PrimeField;

const T: usize = 3;
const FULL_ROUNDS: usize = 8;
const PARTIAL_ROUNDS: usize = 57;

pub fn poseidon_hash(inputs: &[&[u8]]) -> [u8; 32] {
    let mut state = vec![F::zero(); T];
    for (i, input) in inputs.iter().enumerate() {
        state[i] = F::from_random_bytes(input).unwrap_or(F::zero());
    }

    poseidon_permutation(&mut state);

    let result = state[0].into_repr().to_bytes_le();
    let mut output = [0u8; 32];
    output.copy_from_slice(&result[..32]);
    output
}

fn poseidon_permutation(state: &mut [F]) {
    for _ in 0..FULL_ROUNDS {
        add_constants(state);
        full_round(state);
    }

    for _ in 0..PARTIAL_ROUNDS {
        add_constants(state);
        partial_round(state);
    }

    for _ in 0..FULL_ROUNDS {
        add_constants(state);
        full_round(state);
    }
}

fn add_constants(state: &mut [F]) {
    for i in 0..T {
        state[i] += F::from(i as u64);
    }
}

fn full_round(state: &mut [F]) {
    for i in 0..T {
        state[i] = state[i].pow([5u64]);
    }
    mix(state);
}

fn partial_round(state: &mut [F]) {
    state[0] = state[0].pow([5u64]);
    mix(state);
}

fn mix(state: &mut [F]) {
    let mut new_state = vec![F::zero(); T];
    for i in 0..T {
        for j in 0..T {
            new_state[i] += state[j] * F::from((i * j) as u64);
        }
    }
    state.copy_from_slice(&new_state);
}
