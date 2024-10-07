use ark_ff::PrimeField;
use ark_ed_on_bls12_381::Fq as F;
use poseidon::Poseidon;

pub struct PoseidonHash {
    poseidon: Poseidon<F>,
}

impl PoseidonHash {
    pub fn new() -> Self {
        let pos = Poseidon::new();
        Self { poseidon: pos }
    }

    pub fn hash(&self, input: &[F]) -> F {
        self.poseidon.hash(input)
    }

    pub fn hash_bytes(&self, input: &[u8]) -> [u8; 32] {
        let field_elements: Vec<F> = input
            .chunks(32)
            .map(|chunk| F::from_le_bytes_mod_order(chunk))
            .collect();
        
        let hash = self.hash(&field_elements);
        hash.into_repr().to_bytes_le()
    }
}

impl Default for PoseidonHash {
    fn default() -> Self {
        Self::new()
    }
}