import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getAccount, getAssociatedTokenAddress, getAssociatedTokenAddressSync, getMint, getOrCreateAssociatedTokenAccount, transfer, transferChecked } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("29p4kvDvCysE7qniZQqYEidSYmAEadPycz1HKLNn7bs2");

// Recipient address
const to = new PublicKey("gyQyxHt1XNvaUJcySyhtv9bRd522HsrXdiyYjeZnAYW");

(async () => {
    try {
        // Get the mint info to check decimals
        const mintInfo = await getMint(connection, mint);
        console.log(`Mint decimals: ${mintInfo.decimals}`);

        // Get or create the ATAs
        const fromAta = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            keypair.publicKey
        );

        const toAta = await getOrCreateAssociatedTokenAccount(
            connection,
            keypair,
            mint,
            to
        );

        console.log(`Source ATA balance: ${fromAta.amount}`);

        // Perform the transfer with correct decimals
        const tx = await transferChecked(
            connection, 
            keypair, 
            fromAta.address,
            mint,
            toAta.address,
            keypair.publicKey,
            1,  // amount
            mintInfo.decimals   // using correct decimals from mint
        );
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}}`)
    }
})();