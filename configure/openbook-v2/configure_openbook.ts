import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { TestProvider } from "../anchor_utils";
import { Market, createMarket } from "./create_markets";
import { MintUtils } from "../general/mint_utils";
import { OpenbookV2 } from "./openbook_v2";
import IDL from '../programs/openbook_v2.json'
import { BN, Program, web3 } from "@project-serum/anchor";

export interface OpenOrders {
    market: PublicKey,
    openOrders: PublicKey
}

export class OpenbookConfigurator {

    anchorProvider: TestProvider;
    mintUtils: MintUtils;
    openbookProgramId: PublicKey;

    constructor(connection: Connection, authority: Keypair, mintUtils: MintUtils, openbookProgramId: PublicKey) {
        this.anchorProvider = new TestProvider(connection, authority);
        this.mintUtils = mintUtils;
        this.openbookProgramId = openbookProgramId;
    }

    public async configureOpenbookV2(mints: PublicKey[]): Promise<Market[]> {
        let quoteMint = mints[0];
        let admin = Keypair.generate();
        return await Promise.all(mints.slice(1).map((mint, index) => createMarket(this.anchorProvider, this.mintUtils, admin, this.openbookProgramId, mint, quoteMint, index)))
    }

    public async configureMarketForUser(user: Keypair, markets: Market[], depositAmount: number) : Promise<OpenOrders[]> {
        let program = new Program<OpenbookV2>(
            IDL as OpenbookV2,
            this.openbookProgramId,
            this.anchorProvider,
        );

        const openOrders = await Promise.all(
            markets.map(async(market) => {
                let accountIndex = new BN(0);
                let [openOrders, _tmp] = PublicKey.findProgramAddressSync([Buffer.from("OpenOrders"), user.publicKey.toBuffer(), market.marketPk.toBuffer(), accountIndex.toBuffer("le", 4)], this.openbookProgramId)

                await program.methods.initOpenOrders(
                    0,
                    64
                ).accounts({
                    openOrdersAccount: openOrders,
                    market: market.marketPk,
                    owner: user.publicKey,
                    payer: this.anchorProvider.publicKey,
                    systemProgram: web3.SystemProgram.programId,
                }).signers([user]).rpc();
                return [market.marketPk, openOrders]
            })
        )

        return openOrders.map(x=> {
            return {
                market : x[0],
                openOrders : x[1],
            }
        })
    }
}