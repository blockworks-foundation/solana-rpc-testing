import { command, number, option, string, run } from 'cmd-ts';

import * as accounts from "./general/accounts"

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

const programs = option({
    type: string,
    defaultValue: () => "programs.json",
    long: 'programs',
    description: "Programs to be loaded in cluster",
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
            programs,
            nbPayers,
            balancePerPayer,
            outFile,
        },
        handler: ({
            endpoint,
            numberOfAccountsToBeCreated,
            authority,
            programs,
            nbPayers,
            balancePerPayer,
            outFile,
        }) => {
            console.log("configuring a new test instance");
            configure(
                endpoint,
                numberOfAccountsToBeCreated,
                authority,
                programs,
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
    authority: String,
    programs: String,
    nbPayers: number,
    balancePerPayer: number,
    outFile: String,
) {

}