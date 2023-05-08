import { Connection, Keypair, LAMPORTS_PER_SOL, SystemProgram, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";

export async function createPayer(connection: Connection, authority: Keypair, balancePerPayer: number): Promise<Keypair> {
    let payer = Keypair.generate();
    let transfer_ix = SystemProgram.transfer({
        fromPubkey: authority.publicKey,
        toPubkey: payer.publicKey,
        lamports: balancePerPayer * LAMPORTS_PER_SOL,
    });
    let tx = new Transaction().add(transfer_ix);
    tx.feePayer = authority.publicKey;
    const bh = await connection.getLatestBlockhash();
    tx.recentBlockhash = bh.blockhash;
    sendAndConfirmTransaction(connection, tx, [authority]);
    return payer
}
