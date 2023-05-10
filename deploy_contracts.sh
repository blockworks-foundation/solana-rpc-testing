#!/usr/bin/env bash

SCRIPT_DIR=$( dirname -- "$0"; )

# compile openbook-v2 and deploy
OPENBOOK_PID=$(solana address -k $SCRIPT_DIR/configure/programs/openbook_v2-keypair.json)
echo "Openbook PID $OPENBOOK_PID"
cd $SCRIPT_DIR/thirdparty/openbook-v2
git submodule update --init

sed 's@BfxZj7ckfRGHxByn7aHgH2puyXhfjAUvULtRjJo4rd8X@'"$OPENBOOK_PID"'@' programs/openbook-v2/src/lib.rs > programs/openbook-v2/src/lib-tmp.rs
rm programs/openbook-v2/src/lib.rs
mv programs/openbook-v2/src/lib-tmp.rs programs/openbook-v2/src/lib.rs

anchor build -- --features enable-gpl
just idl
cp target/deploy/openbook_v2.so ../../configure/programs/
cp target/idl/openbook_v2.json ../../configure/programs/
cp target/types/openbook_v2.ts ../../configure/openbook-v2/