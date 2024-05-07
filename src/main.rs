mod balances;
mod system;

// This is our main Runtime.
// It accumulates all of the different pallets we want to use.
pub struct Runtime {
	sys: system::Pallet,
    bal: balances::Pallet,
}

impl Runtime {
	// Create a new instance of the main Runtime, by creating a new instance of each pallet.
	fn new() -> Self {
		Self {
            sys: system::Pallet::new(),
            bal: balances::Pallet::new(),
        }
	}
}

fn main() {
    // initialize runtime
    let alice = &"alice".to_string();
    let bob = &"bob".to_string();
    let charlie = &"charlie".to_string();
    let mut r = Runtime::new();
    r.bal.set_balance(alice, 100);

	// start emulating a block
    r.sys.inc_block_number();
    assert_eq!(r.sys.block_number(), 1);

	// first transaction
    r.sys.inc_nonce(alice);
    let res = r
        .bal
        .transfer(alice.to_string(), bob.to_string(), 30)
        .map_err(|e| eprintln!("{}", e));
    assert!(res.is_ok());

	// second transaction
	r.sys.inc_nonce(alice);
    let res = r
        .bal
        .transfer(alice.to_string(), charlie.to_string(), 30)
        .map_err(|e| eprintln!("{}", e));
    assert!(res.is_ok());
}
