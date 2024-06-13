use risc0_zkvm::guest::env;
use alloy_primitives::{Address, B256};
use reth_primitives::{Account, Signature};

fn main() {
    let nonce: u64 = env::read();

    let account: Account = env::read();

    assert_eq!(nonce, account.nonce);

    let output = nonce + account.balance.to::<u64>();

    let signature: Signature = env::read();
    let hash: B256 = env::read();
    let address: Address = env::read();
    let signer = signature.recover_signer(hash).unwrap();


    assert_eq!(address, signer);

    env::commit(&output);
}
