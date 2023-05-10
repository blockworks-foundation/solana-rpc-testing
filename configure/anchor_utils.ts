import { AnchorProvider, Provider } from "@project-serum/anchor";
import { SuccessfulTxSimulationResponse } from "@project-serum/anchor/dist/cjs/utils/rpc";
import { Connection, PublicKey, Transaction, Signer, SendOptions, ConfirmOptions, Commitment, Keypair } from "@solana/web3.js";

export function getProviderFromKeypair(connection: Connection, authority: Keypair) : AnchorProvider {
    let txSigner = async (tx: Transaction) => {
        tx.partialSign(authority);
        return tx
      };
  
      let allSigner = async (txs : Transaction[]) => {
        txs.forEach(x=> x.partialSign(authority));
        return txs;
      };
  
      return new AnchorProvider(connection, 
        {
            signTransaction: txSigner,
            signAllTransactions: allSigner,
            publicKey : authority.publicKey, 
        }, 
        {commitment: 'confirmed'}
      )
}