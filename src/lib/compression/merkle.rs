use crate::compression::poseidon::PoseidonHash;
use ark_ed_on_bls12_381::Fq as F;

pub struct MerkleTree {
    layers: Vec<Vec<[u8; 32]>>,
    hasher: PoseidonHash,
}

impl MerkleTree {
    pub fn new(leaves: Vec<[u8; 32]>) -> Self {
        let mut tree = Self {
            layers: vec![leaves],
            hasher: PoseidonHash::new(),
        };
        tree.build_tree();
        tree
    }

    fn build_tree(&mut self) {
        while self.layers.last().unwrap().len() > 1 {
            let current_layer = self.layers.last().unwrap();
            let mut new_layer = Vec::new();

            for chunk in current_layer.chunks(2) {
                if chunk.len() == 2 {
                    let left = F::from_le_bytes_mod_order(&chunk[0]);
                    let right = F::from_le_bytes_mod_order(&chunk[1]);
                    let parent = self.hasher.hash(&[left, right]);
                    new_layer.push(parent.into_repr().to_bytes_le());
                } else {
                    new_layer.push(chunk[0]);
                }
            }

            self.layers.push(new_layer);
        }
    }

    pub fn root(&self) -> [u8; 32] {
        self.layers.last().unwrap()[0]
    }

    pub fn generate_proof(&self, index: usize) -> Vec<[u8; 32]> {
        let mut proof = Vec::new();
        let mut current_index = index;

        for layer in &self.layers[..self.layers.len() - 1] {
            let sibling_index = if current_index % 2 == 0 { current_index + 1 } else { current_index - 1 };
            if sibling_index < layer.len() {
                proof.push(layer[sibling_index]);
            }
            current_index /= 2;
        }

        proof
    }

    pub fn verify_proof(&self, proof: &[[u8; 32]], leaf: [u8; 32], index: usize) -> bool {
        let mut current = leaf;
        let mut current_index = index;

        for sibling in proof {
            let (left, right) = if current_index % 2 == 0 {
                (current, *sibling)
            } else {
                (*sibling, current)
            };

            let left_f = F::from_le_bytes_mod_order(&left);
            let right_f = F::from_le_bytes_mod_order(&right);
            current = self.hasher.hash(&[left_f, right_f]).into_repr().to_bytes_le();
            current_index /= 2;
        }

        current == self.root()
    }
}