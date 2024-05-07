use std::{collections::BTreeMap, ops::AddAssign};

use num::{One, Zero};

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet<BlockNumber, AccountID,   Nonce> {
    /// The current block number.
    block_number: BlockNumber,
    /// A map from an account to their nonce.
    nonce: BTreeMap<AccountID, Nonce>,
}

impl<BlockNumber, AccountID, Nonce> Pallet<BlockNumber, AccountID, Nonce>
where
    BlockNumber: Zero + One + AddAssign + Copy,
    AccountID: Ord + Clone,
    Nonce: Zero + One + AddAssign + Copy,
{
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: BlockNumber::zero(),
            nonce: BTreeMap::new(),
        }
    }
    
    /// Get the current block number.
	pub fn block_number(&self) -> BlockNumber {
		self.block_number
	}

	// This function can be used to increment the block number.
	// Increases the block number by one.
	pub fn inc_block_number(&mut self) {
		self.block_number.add_assign(BlockNumber::one());
	}

    // Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &AccountID) {
        let mut val = *self.nonce.get(who).unwrap_or(&Nonce::zero());
        val.add_assign(Nonce::one());
        self.nonce.insert(who.clone(), val);
	}
}

#[cfg(test)]
mod test {
    use crate::system::Pallet;

	#[test]
	fn init_system() {
        let mut s: Pallet<u32, String, u32> = Pallet::new();
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
