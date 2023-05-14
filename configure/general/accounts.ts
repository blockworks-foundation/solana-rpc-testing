import {
  Connection,
  Keypair,
  SystemProgram,
  PublicKey,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";

export async function configure_accounts(
  connection: Connection,
  authority: Keypair,
  count: number,
  programs: PublicKey[]
): Promise<PublicKey[]> {
  let all_accounts: PublicKey[] = [];
  // create accounts in batches of 16
  for (let i = 0; i < count; i += 16) {
    let end = Math.min(i + 16, count);
    let nbOfAccs = end - i;
    let accounts = await Promise.all(
      Array.from(Array(nbOfAccs).keys()).map(async (_) => {
        let size = Math.random() * 10_000_000;
        if (size < 100) {
          size = 100;
        }
        size = Math.floor(size);

        const lamports = await connection.getMinimumBalanceForRentExemption(
          size
        );
        let kp = Keypair.generate();
        const program = programs[Math.floor(Math.random() * programs.length)];

        const transaction = new Transaction().add(
          SystemProgram.createAccount({
            fromPubkey: authority.publicKey,
            newAccountPubkey: kp.publicKey,
            lamports,
            space: size,
            programId: program,
          })
        );

        transaction.feePayer = authority.publicKey;
        let hash = await connection.getRecentBlockhash();
        transaction.recentBlockhash = hash.blockhash;
        // Sign transaction, broadcast, and confirm
        await sendAndConfirmTransaction(
          connection,
          transaction,
          [authority, kp],
          { commitment: "confirmed" }
        );

        return kp.publicKey;
      })
    );
    all_accounts = all_accounts.concat(accounts);
  }
  return all_accounts;
}
