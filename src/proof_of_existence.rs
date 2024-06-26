use crate::support::DispatchResult;
use core::fmt::Debug;
use std::collections::BTreeMap;

pub trait Config: crate::system::Config {
    /// The type which represents the content that can be claimed using this pallet.
    /// Could be the content directly as bytes, or better yet the hash of that content.
    /// We leave that decision to the runtime developer.
    type Content: Debug + Ord;
}

/// This is the Proof of Existence Module.
/// It is a simple module that allows accounts to claim existence of some data.
#[derive(Debug)]
pub struct Pallet<T: Config> {
    /// A simple storage map from content to the owner of that content.
    /// Accounts can make multiple different claims, but each claim can only have one owner.
    claims: BTreeMap<T::Content, T::AccountId>,
}

impl<T: Config> Pallet<T> {
    /// Create a new instance of the Proof of Existence Module.
    pub fn new() -> Self {
        Self {
            claims: BTreeMap::new(),
        }
    }

    /// Get the owner (if any) of a claim.
    pub fn get_claim(&self, claim: &T::Content) -> Option<&T::AccountId> {
        self.claims.get(claim)
    }
}

#[macros::call]
impl<T:Config> Pallet<T> {
        /// Create a new claim on behalf of the `caller`.
    /// This function will return an error if someone already has claimed that content.
    pub fn create_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        if self.get_claim(&claim).is_some() {
            return Err("this content has already been claimed");
        }
        self.claims.insert(claim, caller);
        Ok(())
    }

    /// Revoke an existing claim on some content.
    /// This function should only succeed if the caller is the owner of an existing claim.
    /// It will return an error if the claim does not exist, or if the caller is not the owner.
    pub fn revoke_claim(&mut self, caller: T::AccountId, claim: T::Content) -> DispatchResult {
        let owner = self.get_claim(&claim).ok_or("claim does not exist")?;
        if *owner != caller {
            return Err("claim does not belong to caller");
        }
        self.claims.remove(&claim);
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::Pallet;

    struct TestConfig;

    impl super::Config for TestConfig {
        type Content = &'static str;
    }

    impl crate::system::Config for TestConfig {
        type AccountId = &'static str;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn basic_proof_of_existence() {
        let mut poe: Pallet<TestConfig> = Pallet::new();
        let test_claim = "my_claim";
        let test_caller = "0x123";

        assert!(poe.claims.is_empty());
        assert!(poe.get_claim(&test_claim).is_none());
        assert!(poe.create_claim(test_caller, test_claim).is_ok());
        assert_eq!(poe.get_claim(&test_claim), Some(&test_caller));
        assert!(poe
            .create_claim(test_caller, test_claim)
            .is_err_and(|e| e == "this content has already been claimed"));

        assert!(poe
            .revoke_claim(test_caller, "not inserted previously")
            .is_err_and(|e| e == "claim does not exist"));
        assert!(poe
            .revoke_claim("0x456", test_claim)
            .is_err_and(|e| e == "claim does not belong to caller"));
        assert!(poe.revoke_claim(test_caller, test_claim).is_ok());
        assert!(poe.get_claim(&test_claim).is_none());
    }
}
