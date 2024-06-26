use plonky2::{hash::hash_types::HashOut, plonk::config::GenericHashOut};
use zk_commit_core::{
    commitment_tree::CommitmentTree as CoreCommitmentTree,
};

#[derive(uniffi::Record)]
pub struct CommitmentTree {
    pub depth: i32,
    pub commitment_root: Vec<u8>,
    pub commitment_tree: Vec<Vec<u8>>,
}

impl CommitmentTree {
    pub fn from_core(commitment_tree: &CoreCommitmentTree) -> Self {
        Self {
            depth: commitment_tree.depth as i32,
            commitment_root: commitment_tree.commitment_root.to_bytes(),
            commitment_tree: commitment_tree.commitment_tree.iter().map(|x| x.to_bytes()).collect(),
        }
    }

    pub fn to_core(&self) -> CoreCommitmentTree {
        CoreCommitmentTree {
            depth: self.depth.try_into().unwrap(),
            commitment_root: HashOut::from_bytes(&self.commitment_root),
            commitment_tree: self.commitment_tree.iter().map(|x| HashOut::from_bytes(x)).collect(),
        }
    }
}
