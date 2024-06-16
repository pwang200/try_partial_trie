use partial_binary_merkle::{PartialMerkleTrie, ID, Hash};
use rand::prelude::*;
use serde::{Deserialize, Serialize};

pub struct FakeRandom {
    rng: ThreadRng,
}

impl FakeRandom {
    pub fn new() -> FakeRandom {
        FakeRandom { rng: thread_rng() }
    }

    pub fn random_hash(&mut self) -> Hash {
        self.rng.gen()
    }

    pub fn random_hashes(&mut self, n: usize) -> Vec<Hash> {
        (0..n).map(|_| self.random_hash()).collect()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Input {
    pub trie: PartialMerkleTrie,
    pub kv: Vec<(ID, Hash)>,
}

impl Input {
    pub fn new(init_size: usize, n_additions: usize) -> Input {
        let mut fr = FakeRandom::new();
        let trie = {
            let keys = fr.random_hashes(init_size);
            let leaves = fr.random_hashes(init_size);
            let items: Vec<(ID, Hash)> = keys.into_iter().zip(leaves).collect();
            let mut tree = PartialMerkleTrie::new();
            tree.insert_or_replace_batch(items);
            tree
        };

        let kv: Vec<(ID, Hash)> = {
            let keys = fr.random_hashes(n_additions);
            let leaves = fr.random_hashes(n_additions);
            keys.into_iter().zip(leaves).collect()
        };

        Input { trie, kv }
    }

    pub fn process(&mut self) -> Option<Hash> {
        if self.trie.verify_partial() {
            //let mut temp: Vec<(ID, Hash)> = vec![];
            self.trie.insert_or_replace_batch(self.kv.drain(..).collect());
            Some(self.trie.root)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_works() {
        let init_size = 20usize;
        let n_additions = 20usize;
        let mut input = Input::new(init_size, n_additions);
        let x = input.process();
        assert!(x.is_some());
    }
}
