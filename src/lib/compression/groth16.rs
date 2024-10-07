use ark_bls12_381::Bls12_381;
use ark_groth16::{Groth16, ProvingKey, VerifyingKey};
use ark_snark::SNARK;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystem};
use ark_ff::PrimeField;
use rand::thread_rng;

pub struct Groth16Prover {
    proving_key: ProvingKey<Bls12_381>,
    verifying_key: VerifyingKey<Bls12_381>,
}

impl Groth16Prover {
    pub fn new<C, F>(circuit: &C) -> anyhow::Result<Self>
    where
        C: ConstraintSynthesizer<F>,
        F: PrimeField,
    {
        let mut rng = thread_rng();
        let (proving_key, verifying_key) = Groth16::<Bls12_381>::circuit_specific_setup(circuit, &mut rng)?;

        Ok(Self {
            proving_key,
            verifying_key,
        })
    }

    pub fn prove<C, F>(&self, circuit: &C) -> anyhow::Result<ark_groth16::Proof<Bls12_381>>
    where
        C: ConstraintSynthesizer<F>,
        F: PrimeField,
    {
        let mut rng = thread_rng();
        let proof = Groth16::<Bls12_381>::prove(&self.proving_key, circuit, &mut rng)?;
        Ok(proof)
    }

    pub fn verify(&self, proof: &ark_groth16::Proof<Bls12_381>, public_inputs: &[F]) -> anyhow::Result<bool>
    where
        F: PrimeField,
    {
        let result = Groth16::<Bls12_381>::verify(&self.verifying_key, public_inputs, proof)?;
        Ok(result)
    }

    pub fn verify_proof<C, F>(&self, circuit: &C, proof: &ark_groth16::Proof<Bls12_381>) -> anyhow::Result<bool>
    where
        C: ConstraintSynthesizer<F>,
        F: PrimeField,
    {
        let cs = ConstraintSystem::<F>::new_ref();
        circuit.generate_constraints(cs.clone())?;
        let public_inputs = cs.instance_assignment().to_vec();
        self.verify(proof, &public_inputs)
    }
}