use risc0_zkvm::guest::env;
use alloy_primitives::{Address, B256};
use reth_primitives::{Account, Signature, U256};

fn main() {
    let nonce: u64 = env::read();

    let account: Account = env::read();

    assert_eq!(nonce, account.nonce);

    let output = nonce + account.balance.to::<u64>();

    env::commit(&output);
}
