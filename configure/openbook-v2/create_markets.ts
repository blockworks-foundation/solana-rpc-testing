import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import IDL from '../programs/openbook_v2.json'
import { AnchorProvider, Idl, Program, web3, BN } from '@project-serum/anchor';
import { createAccount } from '../general/solana_utils';
import { MintUtils } from '../general/mint_utils';
import { I80F48, I80F48Dto } from '@blockworks-foundation/mango-v4';
import { OpenbookV2 } from './openbook_v2';

export interface Market {
    name: string,
    admin : number[],
    marketPk: PublicKey
    oracle: PublicKey,
    asks: PublicKey,
    bids: PublicKey,
    eventQueue: PublicKey,
    baseVault: PublicKey,
    quoteVault: PublicKey,
    baseMint: PublicKey,
    quoteMint: PublicKey,
    marketIndex: number,
}

export async function createMarket(anchorProvider: AnchorProvider, mintUtils: MintUtils, adminKp: Keypair, openbookProgramId: PublicKey, baseMint: PublicKey, quoteMint: PublicKey, payer: Keypair, index: number): Promise<Market> {
    let program = new Program<OpenbookV2>(
        IDL as OpenbookV2,
        openbookProgramId,
        anchorProvider,
    );
    let [oracleId, _tmp] = PublicKey.findProgramAddressSync([Buffer.from("StubOracle"), baseMint.toBytes()], openbookProgramId)
    const admin:PublicKey = adminKp.publicKey;

    await program.methods.stubOracleCreate({ val: I80F48.fromNumber(1.0).getData() })
    .accounts({
        oracle: oracleId,
        admin,
        mint: baseMint,
        payer: payer.publicKey,
        systemProgram: web3.SystemProgram.programId,
    })
    .signers([adminKp, payer])
    .rpc();

    // bookside size = 123720
    let asks = await createAccount(anchorProvider.connection, payer, 123720, openbookProgramId);
    let bids = await createAccount(anchorProvider.connection, payer, 123720, openbookProgramId);
    let eventQueue = await createAccount(anchorProvider.connection, payer, 97688, openbookProgramId);
    let marketIndex : BN = new BN(index);

    let [marketPk, _tmp2] = PublicKey.findProgramAddressSync([Buffer.from("Market"), marketIndex.toBuffer("le", 4)], openbookProgramId)

    let baseVault = await mintUtils.createTokenAccount(baseMint, payer, marketPk);
    let quoteVault = await mintUtils.createTokenAccount(quoteMint, payer, marketPk);
    let name = 'token at index ' + index.toString() + ' wrt at index 0';

    await program.methods.createMarket(
        marketIndex, 
        name,
        {
                confFilter: 0,
                maxStalenessSlots: 100,
        },
        new BN(1000), new BN(1000), 0, 0, 0
    ).accounts(
        {
            admin,
            market: marketPk,
            bids,
            asks,
            eventQueue,
            payer: payer.publicKey,
            baseVault,
            quoteVault,
            baseMint,
            quoteMint,
            systemProgram: web3.SystemProgram.programId,
            oracle: oracleId,
        }
    ).signers([adminKp, payer])
    .rpc();

    return {
        admin: Array.from(adminKp.secretKey),
        name,
        bids,
        asks,
        eventQueue,
        baseMint,
        baseVault,
        marketIndex,
        marketPk,
        oracle: oracleId,
        quoteMint,
        quoteVault,
    }
}