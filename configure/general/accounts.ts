import {
    Connection,
    Keypair,
    SystemProgram,
    PublicKey,
    Transaction,
    sendAndConfirmTransaction,
} from "@solana/web3.js";

export class AccountGenerator {
    private connection: Connection;
    private feePayer: Keypair;

    public STABLE_SIZE = 7340032; // 7 MB

    constructor(connection: Connection, feePayer: Keypair) {
        this.connection = connection;
        this.feePayer = feePayer;
    }


    async find_max_size_fetchable(): Promise<number> {
        // create Solana accounts till a size of 5 mega bytes
        let good_size = 0;

        for (let i = 1; i < 10; i++) {
            const size = i * 1024 * 1024;

            const account = await this.createSolanaAccount(size);
            // get account info
            try {
                const accountInfo = await this.connection.getAccountInfo(account.publicKey);
                good_size = size;

                console.log(`account size possible ${accountInfo?.data.length} Bytes`);
            } catch (err) {
                console.log(`maximum possible size is ${i - 1} MB or ${good_size} Bytes`, err);
                break;
            }
        }

        return good_size;
    }

    async generate_fetchable_accounts(amount: number): Promise<Keypair[]> {

        return await Promise.all(Array.from(Array(amount).keys()).map(async (i) => {
            const size = this.STABLE_SIZE + (i * 1024); // add a KB to each account
            return await this.createSolanaAccount(size);
        }));
    }

    async createSolanaAccount(space: number): Promise<Keypair> {
        // Generate a new keypair for the account
        const accountKeyPair = Keypair.generate();

        // Fetch the minimum required balance for creating an account
        const minimumBalance = await this.connection.getMinimumBalanceForRentExemption(space);

        // Build the transaction to create the account
        const transaction = new Transaction().add(
            SystemProgram.createAccount({
                fromPubkey: this.feePayer.publicKey,
                newAccountPubkey: accountKeyPair.publicKey,
                lamports: minimumBalance,
                space,
                programId: new PublicKey('11111111111111111111111111111111'), // Replace with the desired program ID
            })
        );

        // send and confirm transaction
        await sendAndConfirmTransaction(this.connection, transaction, [this.feePayer, accountKeyPair]);

        return accountKeyPair;
    }
}

