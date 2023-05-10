import { Connection, Keypair, PublicKey } from "@solana/web3.js";
import { getProviderFromKeypair } from "../anchor_utils";
import { Market, createMarket } from "./create_markets";
import { MintUtils } from "../general/mint_utils";

export async function configureOpenbookV2(connection: Connection, authority: Keypair, mintUtils: MintUtils, mints: PublicKey[], openbookProgramId: PublicKey): Promise<Market[]> {
    
    let anchorProvider = getProviderFromKeypair(connection, authority);
    let quoteMint = mints[0];
    let admin = Keypair.generate();
    return await Promise.all(mints.slice(1).map((mint, index) => createMarket(anchorProvider, mintUtils, admin, openbookProgramId, mint, quoteMint, authority, index)))
}