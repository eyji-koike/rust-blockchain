use crate::prelude::*;

pub struct MerkleTree {
    nodes: Vec<Vec<Hash>>,
}

impl MerkleTree {
    pub fn build<T: Serialize>(items: &[T]) -> Self {
        let mut leaves: Vec<Hash> = items
            .iter()
            .map(|item| {
                let bytes = bincode::serialize(item).unwrap();
                let mut hasher = Sha3_256::new();
                hasher.update(&bytes);
                hasher.finalize().into()
            })
            .collect();

        if leaves.is_empty() {
            leaves.push([0u8; 32]);
        }
        
        while leaves.len() & (leaves.len().wrapping_sub(1)) != 0 {
            leaves.push([0u8; 32]); // Pad with zero hash if not a power of two
        }

        let mut nodes = vec![leaves.clone()];
        while nodes.last().unwrap().len() > 1 {
            let level: Vec<Hash> = nodes.last().unwrap()
                .chunks(2)
                .map(|pair| {
                    let mut hasher = Sha3_256::new();
                    hasher.update(&pair[0]);
                    hasher.update(&pair[1]);
                    hasher.finalize().into()
                })
                .collect();
            nodes.push(level);
        }

        Self { nodes }
    }

    pub fn root(&self) -> Hash {
        self.nodes.last().unwrap()[0]
    }

    pub fn proof(&self, index: usize) -> Vec<(Hash, bool)> {
        let mut proof = Vec::new();
        let mut idx = index;
        for level in &self.nodes[..self.nodes.len() - 1] {
            let sibling_idx = if idx % 2 == 0 { idx + 1 } else { idx - 1 };
            if sibling_idx < level.len() {
                proof.push((level[sibling_idx], idx % 2 == 0));
            }
            idx /= 2;
        }
        proof
    } 
}