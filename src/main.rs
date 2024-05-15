mod balances;
mod support;
mod system;
mod proof_of_existence;

use support::Dispatch;

// These are the concrete types we will use in our simple state machine.
// Modules are configured for these types directly, and they satisfy all of our
// trait requirements.
mod types {
    pub type AccountId = String;
    pub type Balance = u128;
    pub type BlockNumber = u32;
    pub type Nonce = u32;
    pub type Content = &'static str;
    pub type Extrinsic = crate::support::Extrinsic<AccountId, crate::RuntimeCall>;
    pub type Header = crate::support::Header<BlockNumber>;
    pub type Block = crate::support::Block<Header, Extrinsic>;
}

// These are all the calls which are exposed to the world.
// Note that it is just an accumulation of the calls exposed by each module.
pub enum RuntimeCall {
    Balances(balances::Call<Runtime>),
    ProofOfExistence(proof_of_existence::Call<Runtime>),
}

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
#[derive(Debug)]
pub struct Runtime {
    sys: system::Pallet<Self>,
    bal: balances::Pallet<Self>,
    poe: proof_of_existence::Pallet<Self>,
}

impl system::Config for Runtime {
    type AccountId = types::AccountId;
    type BlockNumber = types::BlockNumber;
    type Nonce = types::Nonce;
}

impl balances::Config for Runtime {
    type Balance = types::Balance;
}

impl proof_of_existence::Config for Runtime {
    type Content = types::Content;
}

impl Runtime {
    // Create a new instance of the main Runtime, by creating a new instance of each pallet.
    fn new() -> Self {
        Self {
            sys: system::Pallet::new(),
            bal: balances::Pallet::new(),
            poe: proof_of_existence::Pallet::new(),
        }
    }

    // Execute a block of extrinsics. Increments the block number.
    fn execute_block(&mut self, block: types::Block) -> support::DispatchResult {
        self.sys.inc_block_number();
        if block.header.block_number != self.sys.block_number() {
            return Err("incoming block number doesn't match with system block number");
        }
        for (i, support::Extrinsic { caller, call }) in block.extrinsics.into_iter().enumerate() {
            self.sys.inc_nonce(&caller);

            let _res = self.dispatch(caller, call).map_err(|e| {
                eprintln!(
                    "Extrinsic Error\n\tBlock Number: {}\n\tExtrinsic Number: {}\n\tError: {}",
                    block.header.block_number, i, e
                )
            });
        }
        Ok(())
    }
}

impl crate::support::Dispatch for Runtime {
    type Caller = <Runtime as system::Config>::AccountId;
    type Call = RuntimeCall;
    // Dispatch a call on behalf of a caller. Increments the caller's nonce.
    //
    // Dispatch allows us to identify which underlying module call we want to execute.
    // Note that we extract the `caller` from the extrinsic, and use that information
    // to determine who we are executing the call on behalf of.
    fn dispatch(
        &mut self,
        caller: Self::Caller,
        runtime_call: Self::Call,
    ) -> support::DispatchResult {
        match runtime_call {
            RuntimeCall::Balances(call) => {
                self.bal.dispatch(caller, call)?;
            },
            RuntimeCall::ProofOfExistence(call) => {
                self.poe.dispatch(caller, call)?;
            }
        }
        Ok(())
    }
}

fn main() {
    // initialize runtime
    let alice = "alice".to_string();
    let bob = "bob".to_string();
    let charlie = "charlie".to_string();
    let mut r = Runtime::new();
    r.bal.set_balance(&alice, 100);

    // execute two transactions in one block
    let block_1 = types::Block {
        header: support::Header { block_number: 1 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::transfer {
                    to: bob.clone(),
                    amount: 69,
                }),
            },
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::Balances(balances::Call::transfer {
                    to: charlie,
                    amount: 31,
                }),
            },
        ],
    };

    let block_2 = types::Block {
        header: support::Header { block_number: 2 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::revoke_claim {
                    claim: "hi, bob"
                })
            },
        ]
    };

    let block_3 = types::Block {
        header: support::Header { block_number: 3 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim {
                    claim: "hi, bob"
                })
            },
        ]
    };

    let block_4 = types::Block {
        header: support::Header { block_number: 4 },
        extrinsics: vec![
            support::Extrinsic {
                caller: bob.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::revoke_claim {
                    claim: "hi, bob"
                })
            },
        ]
    };

    let block_5 = types::Block {
        header: support::Header { block_number: 5 },
        extrinsics: vec![
            support::Extrinsic {
                caller: alice.clone(),
                call: RuntimeCall::ProofOfExistence(proof_of_existence::Call::create_claim {
                    claim: "hi, bob"
                })
            },
        ]
    };

    r.execute_block(block_1).expect("invalid block");
    r.execute_block(block_2).expect("invalid block");
    r.execute_block(block_3).expect("invalid block");
    r.execute_block(block_4).expect("invalid block");
    r.execute_block(block_5).expect("invalid block");

    println!("{:#?}", r);
}
