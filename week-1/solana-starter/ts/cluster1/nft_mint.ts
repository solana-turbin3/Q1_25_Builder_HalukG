import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createSignerFromKeypair, signerIdentity, generateSigner, percentAmount } from "@metaplex-foundation/umi"
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../wba-wallet.json"
import base58 from "bs58";

const RPC_ENDPOINT = "https://api.devnet.solana.com";
const umi = createUmi(RPC_ENDPOINT);

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata())

const mint = generateSigner(umi);

(async () => {
    let tx = createNft(umi, {mint, name: "Rug", symbol: "RUG", uri: "https://devnet.irys.xyz/9HC7R7fjGbR5oWoAsqnTRRiJSEgWrktdaHp41is5EfHT", sellerFeeBasisPoints: percentAmount(5)})
    let result = await tx.sendAndConfirm(umi);
    const signature = base58.encode(result.signature);
    
    // console.log(`Succesfully Minted! Check out your TX here:\nhttps://explorer.solana.com/tx/${signature}?cluster=devnet`)

    console.log("Mint Address: ", mint.publicKey);
})();

// Check out my Rug here: https://solscan.io/account/bkc5k2hDf1GgKiuWDaacDK554LD6oZ3k94QGuvgZFeB?cluster=devnet