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
    pub fn new(init_size: usize, n_additions: usize, partial: bool) -> Input {
        let mut fr = FakeRandom::new();
        let mut trie = {
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

        let mut ids = Vec::new();
        kv.iter().for_each(|(k, _v)| ids.push(k) );
        if partial {
            trie = trie.get_partial(&ids);
        }

        Input {trie, kv }
    }

    pub fn verify_and_process(&mut self) -> Option<Hash> {
        if self.trie.verify_partial() {
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
        let mut input = Input::new(0, 5, true);
        let x = input.verify_and_process();
        assert!(x.is_some());

        let mut input = Input::new(1, 50, true);
        let x = input.verify_and_process();
        assert!(x.is_some());

        let mut input = Input::new(100, 100, false);
        let x = input.verify_and_process();
        assert!(x.is_some());

        let mut input = Input::new(10000, 50, true);
        let x = input.verify_and_process();
        assert!(x.is_some());
    }
}
