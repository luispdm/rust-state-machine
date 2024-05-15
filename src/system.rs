use std::{collections::BTreeMap, ops::AddAssign};

use num::{One, Zero};

pub trait Config {
    type BlockNumber: Zero + One + AddAssign + Copy;
    type AccountId: Ord + Clone;
    type Nonce: Zero + One + AddAssign + Copy;
}

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// The current block number.
    block_number: T::BlockNumber,
    /// A map from an account to their nonce.
    nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: T::BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }

    /// Get the current block number.
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }

    // This function can be used to increment the block number.
    // Increases the block number by one.
    pub fn inc_block_number(&mut self) {
        self.block_number.add_assign(T::BlockNumber::one());
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &T::AccountId) {
        let mut val = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        val.add_assign(T::Nonce::one());
        self.nonce.insert(who.clone(), val);
    }
}

#[cfg(test)]
mod test {
    use crate::system::{Config, Pallet};

    struct TestConfig;
    impl Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system() {
        let mut s: Pallet<TestConfig> = Pallet::new();
        assert_eq!(s.block_number(), 0);
        let alice = &"alice".to_string();

        s.inc_block_number();
        s.inc_nonce(alice);

        assert_eq!(s.block_number(), 1);
        assert_eq!(s.nonce.get(alice), Some(&1));

        s.inc_block_number();
        s.inc_nonce(alice);

        assert_eq!(s.block_number(), 2);
        assert_eq!(s.nonce.get(alice), Some(&2));
    }
}
