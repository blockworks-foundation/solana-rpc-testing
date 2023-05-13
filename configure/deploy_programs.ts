import { Connection, Keypair, LAMPORTS_PER_SOL, SystemProgram, PublicKey, Transaction, sendAndConfirmTransaction } from "@solana/web3.js";
import * as web3 from "@solana/web3.js";
import { exec } from "child_process";
import * as fs from "fs"
import { promisify } from "util";
import { getKeypairFromFile } from "./common_utils";

export interface ProgramData {
    name: string,
    programPath: string,
    programKeyPath: string,
    idl: string,
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

        if (program.idl.length > 0) {
            let programId = getKeypairFromFile(program.programKeyPath);
            console.log("deploying idl file for program " + programId.publicKey);
            let initCmd =  "anchor idl init --filepath " + program.idl +  " --provider.wallet " + payer + " --provider.cluster " + url + " "  + programId.publicKey;
            const {stdout, stderr} = await execPromise(initCmd);

            if (stderr.length > 0) {
                let updateCommand =  "anchor idl update --filepath " + program.idl +  " --provider.wallet " + payer + " --provider.cluster " + url + " "  + programId.publicKey;
                const {stdout, stderr} = await execPromise(updateCommand);
                if (stdout.length > 0) {
                    console.log(stdout);
                }

                if(stderr.length > 0) {
                    console.log("could not deploy idl for " + programId.publicKey);
                }
            }
        }
    }
}
