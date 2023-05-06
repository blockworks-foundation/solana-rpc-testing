import { Cluster, Connection, Keypair, LAMPORTS_PER_SOL, SystemProgram, PublicKey, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import * as web3 from "@solana/web3.js"


export async function configure_accounts(connection: Connection, authority: Keypair, count: number, programs: PublicKey[], size: number = 10_000_000) : Promise<PublicKey[]> {
    let accounts = await Promise.all(Array.from(Array(count).keys()).map(async (x) => {

        const lamports = await connection.getMinimumBalanceForRentExemption(size);
        let kp = Keypair.generate();
        const program = programs[Math.floor(Math.random() * programs.length)];

        const transaction = new Transaction().add(
            SystemProgram.createAccount({
                fromPubkey: authority.publicKey,
                newAccountPubkey: kp.publicKey,
                lamports,
                space: size,
                programId: program,
            }))

        transaction.feePayer = authority.publicKey;
        let hash = await connection.getRecentBlockhash();
        transaction.recentBlockhash = hash.blockhash;
        // Sign transaction, broadcast, and confirm
        await sendAndConfirmTransaction(
            connection,
            transaction,
            [authority, kp],
            { commitment: 'confirmed' },
        );
        
        return kp.publicKey
    }))
    return accounts
}