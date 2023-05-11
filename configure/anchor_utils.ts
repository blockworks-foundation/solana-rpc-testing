import { AnchorProvider, Provider } from "@project-serum/anchor";
import { SuccessfulTxSimulationResponse } from "@project-serum/anchor/dist/cjs/utils/rpc";
import { Connection, PublicKey, Transaction, Signer, SendOptions, ConfirmOptions, Commitment, Keypair } from "@solana/web3.js";

export class TestProvider extends AnchorProvider {
  keypair: Keypair;
  constructor(connection: Connection, keypair: Keypair) {
    let txSigner = async (tx: Transaction) => {
      tx.partialSign(this.keypair);
      return tx
    };

    let allSigner = async (txs : Transaction[]) => {
      txs.forEach(x=> x.partialSign(this.keypair));
      return txs;
    };

    super(
      connection, 
      {
        signTransaction: txSigner,
        signAllTransactions: allSigner,
        publicKey : keypair.publicKey, 
      },
      {commitment: 'confirmed'}
    )
    this.keypair = keypair;
  }
  getKeypair() : Keypair {
    return this.keypair
  }
}