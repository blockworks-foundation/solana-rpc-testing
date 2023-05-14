import {
  Connection,
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { Market } from "../openbook-v2/create_markets";
import { MintUtils } from "./mint_utils";
import * as splToken from "@solana/spl-token";
import { OpenOrders } from "../openbook-v2/configure_openbook";

export interface User {
  secret: number[];
  token_data: TokenAccountData[];
  open_orders: OpenOrders[];
}

export async function createUser(
  connection: Connection,
  authority: Keypair,
  balancePerPayer: number
): Promise<Keypair> {
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
  return payer;
}

interface TokenAccountData {
  mint: PublicKey;
  token_account: PublicKey;
}

export async function mintUser(
  connection: Connection,
  authority: Keypair,
  mints: PublicKey[],
  mintUtils: MintUtils,
  user: PublicKey,
  amount: number
): Promise<TokenAccountData[]> {
  return await Promise.all(
    mints.map(async (mint) => {
      const tokenAccount = await mintUtils.createTokenAccount(
        mint,
        authority,
        user
      );
      await splToken.mintTo(
        connection,
        authority,
        mint,
        tokenAccount,
        authority,
        amount
      );
      return {
        mint: mint,
        token_account: tokenAccount,
      };
    })
  );
}
