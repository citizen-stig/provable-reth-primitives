use std::str::FromStr;
use alloy_primitives::{Address, B256};
use reth_primitives::{Account, Signature, U256};
// These constants represent the RISC-V ELF and the image ID generated by risc0-build.
// The ELF is used for proving and the ID is used for verification.
use methods::{
    RETH_PRIMITIVES_GUEST_ELF, RETH_PRIMITIVES_GUEST_ID,
};
use risc0_zkvm::{default_prover, ExecutorEnv};

fn main() {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::filter::EnvFilter::from_default_env())
        .init();

    // An executor environment describes the configurations for the zkVM
    // including program inputs.
    // An default ExecutorEnv can be created like so:
    // `let env = ExecutorEnv::builder().build().unwrap();`
    // However, this `env` does not have any inputs.
    //
    // To add guest input to the executor environment, use
    // ExecutorEnvBuilder::write().
    // To access this method, you'll need to use ExecutorEnv::builder(), which
    // creates an ExecutorEnvBuilder. When you're done adding input, call
    // ExecutorEnvBuilder::build().

    // For example:
    // ---------------------------------------------------------------------------------------------
    // ---------------------------------------------------------------------------------------------
    let nonce: u64 = 15;

    let account = Account {
        nonce,
        balance: U256::from(10_000),
        bytecode_hash: None,
    };

    let signature = Signature {
        r: U256::from_str(
            "18515461264373351373200002665853028612451056578545711640558177340181847433846",
        )
            .unwrap(),
        s: U256::from_str(
            "46948507304638947509940763649030358759909902576025900602547168820602576006531",
        )
            .unwrap(),
        odd_y_parity: false,
    };
    let hash =
        B256::from_str("daf5a779ae972f972197303d7b574746c7ef83eadac0f2791ad23db92e4c8e53")
            .unwrap();
    let signer = signature.recover_signer(hash).unwrap();
    let expected = Address::from_str("0x9d8a62f656a8d1615c1294fd71e9cfb3e4855a4f").unwrap();

    assert_eq!(expected, signer);

    // ---------------------------------------------------------------------------------------------
    // ---------------------------------------------------------------------------------------------


    let env = ExecutorEnv::builder()
        .write(&nonce)
        .unwrap()
        .write(&account)
        .unwrap()
        .write(&signature)
        .unwrap()
        .write(&hash)
        .unwrap()
        .write(&expected)
        .unwrap()
        .build()
        .unwrap();


    // Obtain the default prover.
    let prover = default_prover();

    // Proof information by proving the specified ELF binary.
    // This struct contains the receipt along with statistics about execution of the guest
    let prove_info = prover
        .prove(env, RETH_PRIMITIVES_GUEST_ELF)
        .unwrap();

    // extract the receipt.
    let receipt = prove_info.receipt;

    // TODO: Implement code for retrieving receipt journal here.

    // For example:
    let output: u32 = receipt.journal.decode().unwrap();

    // The receipt was verified at the end of proving, but the below code is an
    // example of how someone else could verify this receipt.
    receipt
        .verify(RETH_PRIMITIVES_GUEST_ID)
        .unwrap();
    tracing::info!(output, "Done.");
}
