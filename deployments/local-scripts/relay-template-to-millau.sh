#!/bin/bash

# A script for relaying Template headers to the Millau chain.
#
# Will not work unless both the Template and Millau are running (see `run-template-node.sh`
# and `run-millau-node.sh).

MILLAU_PORT="${MILLAU_PORT:-9945}"
TEMPLATE_PORT="${TEMPLATE_PORT:-9944}"

RUST_LOG=bridge=debug \
./target/debug/substrate-relay init-bridge TemplateToMillau \
	--target-host localhost \
	--target-port $MILLAU_PORT \
	--source-host localhost \
	--source-port $TEMPLATE_PORT \
	--target-signer //Bob \

sleep 5
RUST_LOG=bridge=debug \
./target/debug/substrate-relay relay-headers TemplateToMillau \
	--target-host localhost \
	--target-port $MILLAU_PORT \
	--source-host localhost \
	--source-port $TEMPLATE_PORT \
	--target-signer //Alice \
	--prometheus-host=0.0.0.0
