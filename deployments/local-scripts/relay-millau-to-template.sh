#!/bin/bash

# A script for relaying Millau headers to the Template chain.
#
# Will not work unless both the Template and Millau are running (see `run-template-node.sh`
# and `run-millau-node.sh).

MILLAU_PORT="${MILLAU_PORT:-9945}"
TEMPLATE_PORT="${TEMPLATE_PORT:-9944}"

RUST_LOG=bridge=debug \
./target/debug/substrate-relay init-bridge MillauToTemplate \
	--source-host localhost \
	--source-port $MILLAU_PORT \
	--target-host localhost \
	--target-port $TEMPLATE_PORT \
	--target-signer //Bob \

sleep 5
RUST_LOG=bridge=debug \
./target/debug/substrate-relay relay-headers MillauToTemplate \
	--source-host localhost \
	--source-port $MILLAU_PORT \
	--target-host localhost \
	--target-port $TEMPLATE_PORT \
	--target-signer //Alice \
	--prometheus-host=0.0.0.0
