import { PublicKey } from "@solana/web3.js";
import { ProgramData } from "./deploy_programs";

export interface OutputFile {
    payers: Uint8Array[],
    programs: ProgramData[],
    known_accounts: PublicKey[],
}