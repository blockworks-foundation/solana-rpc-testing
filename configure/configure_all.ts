import { command, number, option, string, run } from 'cmd-ts';

import * as fs from 'fs';

import programs from './programs.json';
import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, Transaction } from '@solana/web3.js';
import { getKeypairFromFile } from './common_utils';
import { deploy_programs } from './deploy_programs';
import { createPayer } from './general/create_payers';
import { configure_accounts } from './general/accounts';
import { OutputFile } from './output_file';
import { getProviderFromKeypair } from './anchor_utils';
import { MintUtils } from './general/mint_utils';
import { configureOpenbookV2 } from './openbook-v2/configure_openbook';

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

const nbMints = option({
  type: number,
  defaultValue: () => 10,
  long: 'number-of-mints',
  short: 'm',
  description: "Number of mints"
});

const outFile = option({
    type: string,
    defaultValue: () => "config.json",
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
            nbMints,
            outFile,
        },
        handler: ({
            endpoint,
            numberOfAccountsToBeCreated,
            authority,
            nbPayers,
            balancePerPayer,
            nbMints,
            outFile,
        }) => {
            console.log("configuring a new test instance");
            configure(
                endpoint,
                numberOfAccountsToBeCreated,
                authority,
                nbPayers,
                balancePerPayer,
                nbMints,
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
    nbMints: number,
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
    if (authorityBalance < requiredBalance) {
        console.log("authority may have low balance balance " + authorityBalance + " required balance " + requiredBalance);
    }

    let programOutputData = programs.map(x => {

        let kp = getKeypairFromFile(x.programKeyPath);
        return {
            name: x.name,
            program_id: kp.publicKey
        }
    })

    let programIds = programOutputData.map(x => {
        return x.program_id
    });

    console.log("starting program deployment");
    await deploy_programs(endpoint, authorityFile.toString(), programs);
    console.log("programs deployed");

    console.log("Creating payers");
    let payers = await Promise.all(Array.from(Array(nbPayers).keys()).map(_ => createPayer(connection, authority, balancePerPayer)));
    console.log("Payers created");

    console.log("Creating accounts")
    let accounts = await configure_accounts(connection, authority, numberOfAccountsToBeCreated, programIds);
    console.log("Accounts created")

    console.log("Creating Mints");
    let mintUtils = new MintUtils(connection, authority);
    let mints = await mintUtils.createMints(nbMints);
    console.log("Mints created")

    console.log("Configuring openbook-v2")
    let index = programs.findIndex(x => x.name === "openbook_v2");
    let openbookProgramId = programOutputData[index].program_id;
    await configureOpenbookV2(connection, authority, mintUtils, mints, openbookProgramId);
    console.log("Finished configuring openbook")

    let outputFile: OutputFile = {
        programs: programOutputData,
        known_accounts: accounts,
        payers: payers.map(x => Array.from(x.secretKey)),
        mints: mints,
    }

    console.log("creating output file")
    fs.writeFileSync(outFile.toString(), JSON.stringify(outputFile));
}
