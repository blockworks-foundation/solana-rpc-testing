# Solana RPC Testing

This repository aims stress / torture test the solana rpc server. This is done
by sending multiple requests to the solana rpc and measure time to serve all the
requests.

Please feel free to add more tests to the repository.

Current tests :

1. getMultipleAccount : Creating multiple requests by N tasks to get 100
   account. The accounts could be exisiting of non-existing.
2. sendTransaction: Send a light weight memo transactions to the server.
3. getSignatureStatuses: Get signature statuses for the light weight memo
   transactions.

## Configuration

The cluster should be configured before testing. The configuraton script will
deploy the necessary programs, create necessary accounts, create few payers and
fund them some SOLs.

The configuration could be done using the script `configure_all.ts`.

Here are arguments required by the script, all the arguments are optional:

```
--url, -u <str>                 - RPC url [optional][default: http://127.0.0.1:8899]
--number-of-accounts <number>   - a number [optional][default: 1024]
--authority, -a <str>           - a string [optional][default: ~/.config/solana/id.json]
--number-of-payers, -p <number> - Number of payers used for testing [optional][default: 10]
--payer-balance, -b <number>    - Balance of payer in SOLs [optional][default: 1 SOLs]
--output-file, -o <str>         - a string [optional][default: config.json]
```

Once the cluster configuration is successfully done we create a json file `config.json`

To configure cluster:

```sh
cd configure
ts-node configure_all.ts -a /home/user/.config/solana/id.json
cd ..
```

## Running tests

After compiling the rust code a seperate executable will be created to test the
solana rpc. We can run individual tests either `accounts-fetching` or
`send-and-confirm-transaction` or we can test all of them in parallel.

```
-c, --config-file <CONFIG_FILE>                  [default: configure/config.json]
    --accounts-fetching                          To test account fetching
    --send-and-confirm-transaction               To test send and confirm transactions
-a, --test-all                                   To test all
-r, --rpc-addr <RPC_ADDR>                        [default: http://127.0.0.1:8899]
-d, --duration-in-seconds <DURATION_IN_SECONDS>  [default: 60]
```

To run all the tests :
```
cargo run -- -a
```