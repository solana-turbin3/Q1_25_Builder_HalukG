import bs58 from 'bs58';

const privateKeyString = "PRIVATE_KEY";

try {
    const decoded = bs58.decode(privateKeyString);
    
    const array = Array.from(decoded);
    
    console.log("Your private key as an array:");
    console.log(array);
    console.log("\nCopy this array into your wallet.json file");
} catch (error) {
    console.error("Error converting private key:", error);
}