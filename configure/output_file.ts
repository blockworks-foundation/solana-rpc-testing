import { PublicKey } from "@solana/web3.js";

export interface ProgramOutputData {
    name: String,
    program_id: PublicKey,
}

export interface OutputFile {
    payers: number[][],
    programs: ProgramOutputData[],
    known_accounts: PublicKey[],
}