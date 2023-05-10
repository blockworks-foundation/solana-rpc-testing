import { Connection, Keypair, LAMPORTS_PER_SOL, SystemProgram, PublicKey, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import * as web3 from "@solana/web3.js";
import { exec } from "child_process";
import * as fs from "fs"
import { promisify } from "util";

export interface ProgramData {
    name: string,
    programPath: string,
    programKeyPath: string,
}

export async function deploy_programs(url: String, payer: string, programs: ProgramData[]) {
    for (const program of programs) {
        let cmd = 'solana program deploy --program-id ' + program.programKeyPath + ' --keypair ' +  payer + ' --url ' + url + ' ' + program.programPath;
        let execPromise = promisify(exec)
        // wait for exec to complete
        const {stdout, stderr} = await execPromise(cmd);
        if (stdout.length > 0) {
            console.log(stdout);
        }

        if (stderr.length > 0) {
            console.log(stderr);
        }
    }
}
