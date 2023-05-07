import { Cluster, Connection, Keypair, LAMPORTS_PER_SOL, SystemProgram, PublicKey, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import * as web3 from "@solana/web3.js";
import * as fs from "fs"

export interface ProgramData {
    name: String,
    programPath : String,
    programKey: Uint8Array,
}

export async function deploy_programs(connection: Connection, payer: Keypair, programs: ProgramData[]) {
    for (const program of programs) {
        let content = fs.readFileSync(program.programPath.toString(), {encoding: null, flag: 'r'});
        // retries to load a program 10 times
        for (let i=1; i <= 10; ++i) {
            let kp = Keypair.fromSecretKey(program.programKey);
            let programLoaded = await web3.BpfLoader.load(connection, payer, kp, content, web3.BPF_LOADER_PROGRAM_ID);
            if (!programLoaded) {
                console.log("program " + program.name + " loaded unsuccessfully ("+ i +"/10)");
            } else {
                break;
            }
        }
    }
}