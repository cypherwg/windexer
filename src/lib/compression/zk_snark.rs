use ark_ff::PrimeField;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};
use ark_snark::SNARK;
use ark_bls12_381::Bls12_381;
use ark_groth16::{Groth16, PreparedVerifyingKey, Proof, ProvingKey, VerifyingKey};

pub struct ZKSnarkProof(pub Proof<Bls12_381>);
pub struct ZKSnarkVerifyingKey(pub PreparedVerifyingKey<Bls12_381>);

pub trait ZKSnarkTrait<F: PrimeField> {
    fn setup<C: ConstraintSynthesizer<F>>(
        circuit: &C,
    ) -> Result<(ProvingKey<Bls12_381>, VerifyingKey<Bls12_381>), SynthesisError>;

    fn prove<C: ConstraintSynthesizer<F>>(
        circuit: &C,
        proving_key: &ProvingKey<Bls12_381>,
    ) -> Result<ZKSnarkProof, SynthesisError>;

    fn verify(
        verifying_key: &ZKSnarkVerifyingKey,
        proof: &ZKSnarkProof,
        public_inputs: &[F],
    ) -> Result<bool, SynthesisError>;
}

pub struct ZKSnark;

impl<F: PrimeField> ZKSnarkTrait<F> for ZKSnark {
    fn setup<C: ConstraintSynthesizer<F>>(
        circuit: &C,
    ) -> Result<(ProvingKey<Bls12_381>, VerifyingKey<Bls12_381>), SynthesisError> {
        Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rand::thread_rng())
    }

    fn prove<C: ConstraintSynthesizer<F>>(
        circuit: &C,
        proving_key: &ProvingKey<Bls12_381>,
    ) -> Result<ZKSnarkProof, SynthesisError> {
        let proof = Groth16::<Bls12_381>::prove(proving_key, circuit, &mut rand::thread_rng())?;
        Ok(ZKSnarkProof(proof))
    }

    fn verify(
        verifying_key: &ZKSnarkVerifyingKey,
        proof: &ZKSnarkProof,
        public_inputs: &[F],
    ) -> Result<bool, SynthesisError> {
        Groth16::<Bls12_381>::verify_with_processed_vk(&verifying_key.0, public_inputs, &proof.0)
    }
}

pub fn prepare_verifying_key(vk: &VerifyingKey<Bls12_381>) -> ZKSnarkVerifyingKey {
    ZKSnarkVerifyingKey(Groth16::<Bls12_381>::process_vk(vk).unwrap())
}