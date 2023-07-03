import { PublicKey } from "@solana/web3.js";
import { Market } from "./openbook-v2/create_markets";
import { User } from "./general/create_users";

export interface Command {
    name: String;
    instruction: number[];
    argument_sizes: number[];
}

export interface ProgramOutputData {
    name: String;
    program_id: PublicKey;
    commands: Command[];
}

export interface OutputFile {
    users: User[];
    programs: ProgramOutputData[];
    known_accounts: PublicKey[];
    mints: PublicKey[];
    markets: Market[];
}
