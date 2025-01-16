pub mod programs;

#[cfg(test)]
mod tests {
    use solana_sdk::{
        signature::{Keypair, Signer, read_keypair_file},
        pubkey::Pubkey,
    };
    use solana_client::rpc_client::RpcClient;
    use std::io::{self, BufRead};
    use bs58;

    const RPC_URL: &str = "https://api.devnet.solana.com";

    // -------------------------------------------
    // 1. Create a new Keypair
    // -------------------------------------------
    #[test]
    fn keygen() {
        use std::fs::File;
        use std::io::Write;
        
        // Create a new keypair
        let kp = Keypair::new();
        
        // Save as dev-wallet.json
        let bytes = kp.to_bytes();
        File::create("dev-wallet.json")
            .expect("Failed to create wallet file")
            .write_all(&bytes)
            .expect("Failed to write wallet file");
        
        // Print wallet info
        println!("\n=== Wallet Generation Success ===");
        println!("Public Key: {}", kp.pubkey());
        println!("\nWallet saved to: dev-wallet.json");
        println!("\nWallet contents (SAVE THIS AS BACKUP):");
        println!("{:?}", bytes);
        println!("===============================\n");
    }

    // -------------------------------------------
    // 1.3 Import/Export to Phantom
    // -------------------------------------------
    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as base58:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a wallet file byte array:");
        let stdin = io::stdin();
        let wallet = stdin.lock().lines().next().unwrap().unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    // -------------------------------------------
    // 2. Claim Token Airdrop
    // -------------------------------------------
    #[test]
    fn airdrop() {
        use std::{thread, time::Duration};
        
        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let client = RpcClient::new(RPC_URL);

        // Check initial balance
        let initial_balance = client.get_balance(&keypair.pubkey()).expect("Failed to get balance");
        println!("Initial balance: {} SOL", initial_balance as f64 / 1_000_000_000.0);

        // Try airdrop with retries
        let mut attempts = 0;
        let max_attempts = 5;
        
        while attempts < max_attempts {
            println!("Attempting airdrop (attempt {}/{})", attempts + 1, max_attempts);
            
            match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
                Ok(sig) => {
                    println!("Airdrop requested, signature: {}", sig);
                    
                    // Wait for confirmation
                    println!("Waiting for confirmation...");
                    thread::sleep(Duration::from_secs(20));
                    
                    // Verify the balance increased
                    let new_balance = client.get_balance(&keypair.pubkey()).expect("Failed to get balance");
                    if new_balance > initial_balance {
                        println!("Airdrop successful!");
                        println!("New balance: {} SOL", new_balance as f64 / 1_000_000_000.0);
                        println!("TX: https://explorer.solana.com/tx/{}?cluster=devnet", sig);
                        return;
                    }
                },
                Err(e) => {
                    println!("Airdrop attempt failed: {}", e);
                }
            }

            attempts += 1;
            if attempts < max_attempts {
                println!("Waiting 20 seconds before retry...");
                thread::sleep(Duration::from_secs(20));
            }
        }

        panic!("Failed to get airdrop after {} attempts", max_attempts);
    }

    // -------------------------------------------
    // 3. Transfer tokens to Turbin3 Address
    // -------------------------------------------
    #[test]
    fn transfer_sol() {
        use solana_program::system_instruction::transfer;
        use solana_sdk::transaction::Transaction;

        // Import our keypair
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        // Define our Turbin3 public key
        let to_pubkey = read_keypair_file("Turbin3-wallet.json")
            .expect("Couldn't find Turbin3 wallet file")
            .pubkey();

        // Create a Solana devnet connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Get recent blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Create transaction
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 100_000_000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        );

        // Send transaction
        match rpc_client.send_and_confirm_transaction(&transaction) {
            Ok(sig) => {
                println!(
                    "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
                    sig
                );
            },
            Err(e) => println!("Transaction failed: {}", e),
        }
    }

    // -------------------------------------------
    // 4. Empty devnet wallet into Turbin3 wallet
    // -------------------------------------------
    #[test]
    fn empty_wallet() {
        use solana_program::system_instruction::transfer;
        use solana_sdk::{transaction::Transaction, message::Message};

        // Import keypairs
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");
        let to_pubkey = read_keypair_file("Turbin3-wallet.json")
            .expect("Couldn't find Turbin3 wallet file")
            .pubkey();

        // Create connection
        let rpc_client = RpcClient::new(RPC_URL);

        // Get balance
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        println!("Current balance: {} SOL", balance as f64 / 1_000_000_000.0);

        // Get blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        // Calculate fee
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash
        );

        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        println!("Transaction fee: {} SOL", fee as f64 / 1_000_000_000.0);

        // Create transaction with fee deducted
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash
        );

        // Send transaction
        match rpc_client.send_and_confirm_transaction(&transaction) {
            Ok(sig) => {
                println!(
                    "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
                    sig
                );
            },
            Err(e) => println!("Transaction failed: {}", e),
        }
    }

    // -------------------------------------------
    // 5. Submit completion of Turbin3 prerequisites
    // -------------------------------------------
    #[test]
    fn enroll_turbin3_prereq() {
        use solana_sdk::system_program;
        use crate::programs::wba_prereq::{WbaPrereqProgram, CompleteArgs, UpdateArgs};
    
        // Create connection
        let rpc_client = RpcClient::new(RPC_URL);
    
        // Load Turbin3 wallet
        let signer = read_keypair_file("Turbin3-wallet.json")
            .expect("Couldn't find Turbin3 wallet file");
    
        // Print wallet info
        println!("Using wallet: {}", signer.pubkey());
        let balance = rpc_client.get_balance(&signer.pubkey()).expect("Failed to get balance");
        println!("Wallet balance: {} SOL", balance as f64 / 1_000_000_000.0);
    
        // Derive PDA
        let prereq = WbaPrereqProgram::derive_program_address(&[
            b"prereq",
            signer.pubkey().to_bytes().as_ref()
        ]);
        println!("Derived PDA: {}", prereq);
    
        // Get blockhash
        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
    
        // Try update first since account likely exists
        println!("Attempting update first...");
        
        let update_args = UpdateArgs {
            github: b"HalukG".to_vec()  // Replace with your actual GitHub username
        };
    
        let update_tx = WbaPrereqProgram::update(
            &[
                &signer.pubkey(),
                &prereq,
                &system_program::id()
            ],
            &update_args,
            Some(&signer.pubkey()),
            &[&signer],
            recent_blockhash
        );
    
        match rpc_client.send_and_confirm_transaction(&update_tx) {
            Ok(sig) => {
                println!("Update successful!");
                println!("Transaction: https://explorer.solana.com/tx/{}/?cluster=devnet", sig);
                return;
            },
            Err(e) => {
                println!("Update failed: {}", e);
                println!("Trying complete instruction instead...");
            }
        }
    
        // If update failed, try complete
        let complete_args = CompleteArgs {
            github: b"HalukG".to_vec()
        };
    
        let complete_tx = WbaPrereqProgram::complete(
            &[
                &signer.pubkey(),
                &prereq,
                &system_program::id()
            ],
            &complete_args,
            Some(&signer.pubkey()),
            &[&signer],
            recent_blockhash
        );
    
        match rpc_client.send_and_confirm_transaction(&complete_tx) {
            Ok(sig) => {
                println!("Complete successful!");
                println!("Transaction: https://explorer.solana.com/tx/{}/?cluster=devnet", sig);
            },
            Err(e) => {
                if e.to_string().contains("custom program error: 0x0") {
                    panic!("Program error: This could mean the prereq account already exists or there's an issue with the program. Try using the program explorer to check the account status: https://explorer.solana.com/address/{}?cluster=devnet", prereq);
                } else {
                    panic!("Complete failed: {}", e);
                }
            }
        }
    }
}