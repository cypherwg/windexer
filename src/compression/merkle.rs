use crate::compression::poseidon::poseidon_hash;

pub struct MerkleTree {
    pub max_depth: usize,
    pub leaves: Vec<[u8; 32]>,
    pub nodes: Vec<Vec<[u8; 32]>>,
}

impl MerkleTree {
    pub fn new(max_depth: usize) -> Self {
        MerkleTree {
            max_depth,
            leaves: Vec::new(),
            nodes: vec![Vec::new(); max_depth + 1],
        }
    }

    pub fn append(&mut self, leaf: [u8; 32]) {
        self.leaves.push(leaf);
        self.update_tree();
    }

    pub fn update(&mut self, index: usize, new_leaf: [u8; 32]) {
        if index < self.leaves.len() {
            self.leaves[index] = new_leaf;
            self.update_tree();
        }
    }

    fn update_tree(&mut self) {
        for i in 0..self.max_depth {
            let level_size = (self.leaves.len() + (1 << i) - 1) >> i;
            self.nodes[i] = Vec::with_capacity(level_size);
            for j in (0..level_size).step_by(2) {
                let left = if j < level_size {
                    self.get_node(i, j)
                } else {
                    [0u8; 32]
                };
                let right = if j + 1 < level_size {
                    self.get_node(i, j + 1)
                } else {
                    [0u8; 32]
                };
                let parent = poseidon_hash(&[&left, &right]);
                self.nodes[i + 1].push(parent);
            }
        }
    }

    fn get_node(&self, level: usize, index: usize) -> [u8; 32] {
        if level == 0 {
            self.leaves[index]
        } else {
            self.nodes[level - 1][index]
        }
    }

    pub fn root(&self) -> [u8; 32] {
        if self.nodes[self.max_depth].is_empty() {
            [0u8; 32]
        } else {
            self.nodes[self.max_depth][0]
        }
    }

    pub fn generate_proof(&self, index: usize) -> Vec<([u8; 32], bool)> {
        let mut proof = Vec::new();
        let mut current_index = index;
        for i in 0..self.max_depth {
            let sibling_index = current_index ^ 1;
            if sibling_index < self.nodes[i].len() {
                let is_left = current_index & 1 == 0;
                proof.push((self.get_node(i, sibling_index), is_left));
            }
            current_index >>= 1;
        }
        proof
    }
}
