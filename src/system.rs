use std::collections::BTreeMap;

type AccountID = String;
type BlockNumber = u32;
type Nonce = u32;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
#[derive(Debug)]
pub struct Pallet {
    /// The current block number.
    block_number: BlockNumber,
    /// A map from an account to their nonce.
    nonce: BTreeMap<AccountID, Nonce>,
}

impl Pallet {
    /// Create a new instance of the System Pallet.
    pub fn new() -> Self {
        Self {
            block_number: 0,
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
		self.block_number+=1;
	}

    // Increment the nonce of an account. This helps us keep track of how many transactions each
	// account has made.
	pub fn inc_nonce(&mut self, who: &AccountID) {
        match self.nonce.get(who) {
            Some(val) => self.nonce.insert(who.to_string(), val+1),
            None => self.nonce.insert(who.to_string(), 1),
        };
	}
}

#[cfg(test)]
mod test {
    use crate::system::Pallet;

	#[test]
	fn init_system() {
        let mut s = Pallet::new();
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
