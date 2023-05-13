import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { TestProvider } from "../anchor_utils";
import { Market, createMarket } from "./create_markets";
import { MintUtils } from "../general/mint_utils";
import { OpenbookV2 } from "./openbook_v2";
import IDL from '../programs/openbook_v2.json'
import { BN, Program, web3, IdlTypes } from "@project-serum/anchor";
import { User } from "../general/create_users";
import { U64_MAX_BN } from "@blockworks-foundation/mango-v4";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { Command } from "../output_file";
import assert from "assert";

export interface OpenOrders {
    market: PublicKey,
    open_orders: PublicKey
}

export class OpenbookConfigurator {

    anchorProvider: TestProvider;
    mintUtils: MintUtils;
    openbookProgramId: PublicKey;
    program: Program<OpenbookV2>;

    constructor(connection: Connection, authority: Keypair, mintUtils: MintUtils, openbookProgramId: PublicKey) {
        this.anchorProvider = new TestProvider(connection, authority);
        this.mintUtils = mintUtils;
        this.openbookProgramId = openbookProgramId;
        this.program = new Program<OpenbookV2>(
            IDL as OpenbookV2,
            this.openbookProgramId,
            this.anchorProvider,
        );
    }

    public async configureOpenbookV2(mints: PublicKey[]): Promise<Market[]> {
        let quoteMint = mints[0];
        let admin = Keypair.generate();
        return await Promise.all(mints.slice(1).map((mint, index) => createMarket(this.program, this.anchorProvider, this.mintUtils, admin, this.openbookProgramId, mint, quoteMint, index)))
    }

    public async configureMarketForUser(user: Keypair, markets: Market[],) : Promise<OpenOrders[]> {

        const openOrders = await Promise.all(
            markets.map(async(market) => {
                let accountIndex = new BN(0);
                let [openOrders, _tmp] = PublicKey.findProgramAddressSync([Buffer.from("OpenOrders"), user.publicKey.toBuffer(), market.market_pk.toBuffer(), accountIndex.toBuffer("le", 4)], this.openbookProgramId)

                await this.program.methods.initOpenOrders(
                    0,
                    64
                ).accounts({
                    openOrdersAccount: openOrders,
                    market: market.market_pk,
                    owner: user.publicKey,
                    payer: this.anchorProvider.publicKey,
                    systemProgram: web3.SystemProgram.programId,
                }).signers([user]).rpc();
                return [market.market_pk, openOrders]
            })
        )

        return openOrders.map(x=> {
            return {
                market : x[0],
                open_orders : x[1],
            }
        })
    }

    public async fillOrderBook(user: User, userKp: Keypair, market: Market, nbOrders: number) {

        for( let i=0; i<nbOrders; ++i) {

            let side = {bid:{}} ;
            let placeOrder  = {limit:{}};

            await this.program.methods.placeOrder(
                side,
                new BN(1000-1-i),
                new BN(10000),
                new BN(1000000),
                new BN(i),
                placeOrder,
                false,
                U64_MAX_BN,
                255,
            ).accounts({
                asks: market.asks,
                baseVault: market.base_vault,
                bids: market.bids,
                eventQueue: market.event_queue,
                market: market.market_pk,
                openOrdersAccount: user.open_orders[market.market_index].open_orders,
                oracle: market.oracle,
                owner: userKp.publicKey,
                payer: user.token_data[0].token_account,
                quoteVault: market.quote_vault,
                systemProgram: web3.SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
            }).signers([userKp]).rpc();
        }

        for( let i=0; i<nbOrders; ++i) {

            let side = {ask:{}} ;
            let placeOrder  = {limit:{}};

            await this.program.methods.placeOrder(
                side,
                new BN(1000+1+i),
                new BN(10000),
                new BN(1000000),
                new BN(i),
                placeOrder,
                false,
                U64_MAX_BN,
                255,
            ).accounts({
                asks: market.asks,
                baseVault: market.base_vault,
                bids: market.bids,
                eventQueue: market.event_queue,
                market: market.market_pk,
                openOrdersAccount: user.open_orders[market.market_index].open_orders,
                oracle: market.oracle,
                owner: userKp.publicKey,
                payer: user.token_data[market.market_index+1].token_account,
                quoteVault: market.quote_vault,
                systemProgram: web3.SystemProgram.programId,
                tokenProgram: TOKEN_PROGRAM_ID,
            }).signers([userKp]).rpc();
        }
    }

    
    /// this is a special method.
    /// It is pain to create an anchor instruction in rust
    /// so this method will create the instruction in typescript and serialize into bytes and store it into command type
    public async getCommands() : Promise<Command[]> {
    let side = {bid:{}} ;
    let placeOrder  = {limit:{}};
    let placeOrderIx = await this.program.methods.placeOrder(
        side,
        new BN(0),
        new BN(0),
        new BN(0),
        new BN(0),
        placeOrder,
        false,
        U64_MAX_BN,
        255,
    ).accounts({
        asks: PublicKey.default,
        baseVault: PublicKey.default,
        bids: PublicKey.default,
        eventQueue: PublicKey.default,
        market: PublicKey.default,
        openOrdersAccount: PublicKey.default,
        oracle: PublicKey.default,
        owner: PublicKey.default,
        payer: PublicKey.default,
        quoteVault: PublicKey.default,
        systemProgram: web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
    }).instruction();

    let argument_sizes = [8, 1, 8, 8, 8, 8, 1, 1, 8, 1];
    assert(argument_sizes.reduce( (sum, current) => sum + current, 0 ) === placeOrderIx.data.length);
    let placeOrderCommand : Command = {
        name: "placeOrder",
        instruction: Array.from(placeOrderIx.data),
        argument_sizes, 
    };

    return [placeOrderCommand]
  }
}