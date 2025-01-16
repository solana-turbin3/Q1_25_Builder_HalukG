import { Connection, Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { Program, Wallet, AnchorProvider, Idl } from "@coral-xyz/anchor";
import { IDL } from "./Turbin3_prereq";
import wallet from "../Turbin3-wallet.json";

(async () => {
  try {
    // 1) Load your local wallet (private key)
    const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

    // 2) Create a devnet connection
    const connection = new Connection("https://api.devnet.solana.com");

    // 3) Create an Anchor provider
    const provider = new AnchorProvider(connection, new Wallet(keypair), {
      commitment: "confirmed",
    });

    // 4) Construct the Program from the IDL only
    //    Anchor 0.30+ will read `IDL.address` for the programId
    const program = new Program(IDL as Idl, provider);

    // Confirm the programId from the IDL
    console.log("Program ID:", program.programId.toBase58());

    // 5) Derive the "prereq" PDA: seeds = ["prereq", signerPublicKey]
    //    ASCII "prereq" => [112, 114, 101, 114, 101, 113]
    const [pda] = PublicKey.findProgramAddressSync(
      [Buffer.from("prereq"), keypair.publicKey.toBuffer()],
      program.programId
    );

    console.log("PDA (prereq account):", pda.toBase58());

    // 6) Convert your GitHub handle to bytes
    const github = Buffer.from("HalukG", "utf8");
    console.log("GitHub (UTF-8):", github.toString());

    // 7) Invoke the "complete" instruction
    //    Note we use system_program: SystemProgram.programId
    //    to match the IDL's third account name.
    const txhash = await program.methods
      .complete(github)
      .accounts({
        signer: keypair.publicKey,
        prereq: pda,
        system_program: SystemProgram.programId
      })
      .signers([keypair])
      .rpc();

    console.log(`
Success! Check out your TX here:
https://explorer.solana.com/tx/${txhash}?cluster=devnet
`);
  } catch (e) {
    console.error(`Oops, something went wrong: ${e}`);
  }
})();