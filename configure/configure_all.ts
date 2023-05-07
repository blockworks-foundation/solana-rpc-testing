import { command, number, option, string, run } from 'cmd-ts';

import * as accounts from "./general/accounts"
import * as fs from 'fs';

import programs from './programs.json';
import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, SystemInstruction, SystemProgram, Transaction, sendAndConfirmTransaction } from '@solana/web3.js';
import { getKeypairFromFile } from './common_utils';
import { deploy_programs } from './deploy_programs';
import { createPayer } from './general/create_payers';
import { configure_accounts } from './general/accounts';
import { OutputFile } from './output_file';

const numberOfAccountsToBeCreated = option({
    type: number,
    defaultValue: () => 1024,
    long: 'number-of-accounts',
  });

const endpoint = option({
    type: string,
    defaultValue: () => "http://127.0.0.1:8899",
    long: 'url',
    short: 'u',
    description: "RPC url",
  });

const authority = option({
    type: string,
    defaultValue: () => "~/.config/solana/id.json",
    long: 'authority',
    short: 'a'
  });

const nbPayers = option({
    type: number,
    defaultValue: () => 10,
    long: 'number-of-payers',
    short: 'p',
    description: "Number of payers used for testing"
  });

const balancePerPayer = option({
    type: number,
    defaultValue: () => 1,
    long: 'payer-balance',
    short: 'b',
    description: "Balance of payer in SOLs"
  });

const outFile = option({
    type: string,
    defaultValue: () => "out.json",
    long: 'output-file',
    short: 'o'
  });

const app = command(
    {
        name: "configure",
        args: {
            endpoint,
            numberOfAccountsToBeCreated,
            authority,
            nbPayers,
            balancePerPayer,
            outFile,
        },
        handler: ({
            endpoint,
            numberOfAccountsToBeCreated,
            authority,
            nbPayers,
            balancePerPayer,
            outFile,
        }) => {
            console.log("configuring a new test instance");
            configure(
                endpoint,
                numberOfAccountsToBeCreated,
                authority,
                nbPayers,
                balancePerPayer,
                outFile,
            ).then(_ => {
                console.log("configuration finished");
            });
          },
    }
)

run(app, process.argv.slice(2))

// configure part
async function configure(
    endpoint: String,
    numberOfAccountsToBeCreated: number,
    authorityFile: String,
    nbPayers: number,
    balancePerPayer: number,
    outFile: String,
) {
    // create connections
    const connection = new Connection(
      endpoint.toString(),
      'confirmed' as Commitment,
    );

    // configure authority
    const authority = getKeypairFromFile(authorityFile);
    const authorityBalance = await connection.getBalance(authority.publicKey);
    const requiredBalance = nbPayers * (balancePerPayer * LAMPORTS_PER_SOL) + 100 * LAMPORTS_PER_SOL;
    if (authorityBalance < requiredBalance ) {
      console.log("authority may have low balance balance " + authorityBalance + " required balance " + requiredBalance );
    }
    
    const programsData = programs.map( x => {
        let programid = getKeypairFromFile(x.kp);
        return {
            name : x.name,
            programPath : x.program,
            keypair: programid, 
        }
    });

    let programIds = programsData.map(x => x.keypair.publicKey);

    console.log("starting program deployment");
    await deploy_programs(connection, authority, programsData);
    console.log("programs deployed");

    console.log("Creating payers");
    let payers = await Promise.all(Array.from(Array(nbPayers).keys()).map(x => {
      return createPayer(connection, authority, balancePerPayer)
    }));
    console.log("Payers created");

    console.log("Creating accounts")
    let accounts = await configure_accounts(connection, authority, numberOfAccountsToBeCreated, programIds);
    console.log("Accounts created")

    let outputFile: OutputFile = {
      programs: programsData,
      known_accounts: accounts,
      payers: payers.map(x => x.secretKey),
    }

    console.log("creating output file")
    fs.writeFileSync(outFile.toString(), JSON.stringify(outputFile));
}