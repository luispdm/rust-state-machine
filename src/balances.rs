use std::collections::BTreeMap;

type AccountID = String;
type Balance = u128;

/// This is the Balances Module.
/// It is a simple module which keeps track of how much balance each account has in this state
/// machine.
#[derive(Debug)]
pub struct Pallet {
    // A simple storage mapping from accounts (`String`) to their balances (`u128`).
	balances: BTreeMap<AccountID, Balance>,
}

impl Pallet {
    /// Create a new instance of the balances module.
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	/// Set the balance of an account `who` to some `amount`.
	pub fn set_balance(&mut self, who: &AccountID, amount: Balance) {
		self.balances.insert(who.to_string(), amount);
	}

	/// Get the balance of an account `who`.
	/// If the account has no stored balance, we return zero.
	pub fn balance(&self, who: &AccountID) -> Balance {
		*self.balances.get(who).unwrap_or(&0)
	}

    /// Transfer `amount` from one account to another.
	/// This function verifies that `from` has at least `amount` balance to transfer,
	/// and that no mathematical overflows occur.
	pub fn transfer(
		&mut self,
		caller: AccountID,
		to: AccountID,
		amount: Balance,
	) -> Result<(), &'static str> {
        let new_from_b = self.
            balance(&caller).
            checked_sub(amount).
            ok_or("Not enough funds.")?;
        let new_to_b = self.
            balance(&to).
            checked_add(amount).
            ok_or("Maximum amount of funds reached")?;
        
        self.set_balance(&caller, new_from_b);
        self.set_balance(&to, new_to_b);
		Ok(())
	}
}

#[cfg(test)]
mod tests {
    use crate::balances::Pallet;

	#[test]
    fn init_balances() {
        let mut b = Pallet::new();
        
        assert_eq!(b.balance(&"alice".to_string()), 0);
        b.set_balance(&"alice".to_string(), 100);
        assert_eq!(b.balance(&"alice".to_string()), 100);
        assert_eq!(b.balance(&"bob".to_string()), 0);
    }

    #[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::new();

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 51),
			Err("Not enough funds.")
		);

		balances.set_balance(&"alice".to_string(), 100);
		assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 51), Ok(()));
		assert_eq!(balances.balance(&"alice".to_string()), 49);
		assert_eq!(balances.balance(&"bob".to_string()), 51);

		assert_eq!(
			balances.transfer("alice".to_string(), "bob".to_string(), 51),
			Err("Not enough funds.")
		);
	}
}
