use solana_client::{rpc_client::RpcClient, rpc_config::RpcSendTransactionConfig};
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use std::str::FromStr;

// Include the smart contract module
include!("lib.rs"); // Assuming lib.rs is in the same directory

pub struct Wallet {
    keypair: Keypair,
    rpc_client: RpcClient,
}

impl Wallet {
    // Initialize a new Wallet with a given RPC URL
    pub fn new(rpc_url: &str) -> Self {
        let rpc_client = RpcClient::new(rpc_url.to_string());
        let keypair = Keypair::new(); // Create a new keypair for the wallet
        Wallet {
            keypair,
            rpc_client,
        }
    }

    // Get the public key of the wallet
    pub fn public_key(&self) -> Pubkey {
        self.keypair.pubkey()
    }

    // Function to send transaction to the Solana contract
    pub fn send_transaction(&self, program_id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let program_pubkey = Pubkey::from_str(program_id)?;

        // Create instruction data here (specific to your contract)
        let instruction_data = vec![]; // Placeholder for instruction data

        // Create the transaction
        let transaction = Transaction::new_signed_with_payer(
            &[],
            Some(&self.keypair.pubkey()),
            &[&self.keypair],
            self.rpc_client.get_latest_blockhash()?,
        );

        // Send the transaction
        self.rpc_client
            .send_and_confirm_transaction_with_spinner_and_config(
                &transaction,
                RpcSendTransactionConfig {
                    skip_preflight: true,
                    ..Default::default()
                },
            )?;

        Ok(())
    }

    // Additional functions for wallet operations can be added here
}
