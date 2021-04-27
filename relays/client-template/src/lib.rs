// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Types used to connect to the Template chain.

use codec::Encode;
use relay_substrate_client::{Chain, ChainBase, ChainWithBalances, TransactionSignScheme};
use sp_core::{storage::StorageKey, Pair};
use sp_runtime::{generic::SignedPayload, traits::IdentifyAccount};
use std::time::Duration;

/// Template header id.
pub type HeaderId = relay_utils::HeaderId<bp_template::Hash, bp_template::BlockNumber>;

/// Template header type used in headers sync.
pub type SyncHeader = relay_substrate_client::SyncHeader<bp_template::Header>;

/// Template chain definition
#[derive(Debug, Clone, Copy)]
pub struct Template;

impl ChainBase for Template {
	type BlockNumber = bp_template::BlockNumber;
	type Hash = bp_template::Hash;
	type Hasher = bp_template::Hasher;
	type Header = bp_template::Header;
}

impl Chain for Template {
	const NAME: &'static str = "Template";
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(6);

	type AccountId = bp_template::AccountId;
	type Index = bp_template::Nonce;
	type SignedBlock = bp_template::SignedBlock;
	type Call = template_runtime::Call;
}

impl ChainWithBalances for Template {
	type NativeBalance = bp_template::Balance;

	fn account_info_storage_key(account_id: &Self::AccountId) -> StorageKey {
		StorageKey(bp_template::account_info_storage_key(account_id))
	}
}

impl TransactionSignScheme for Template {
	type Chain = Template;
	type AccountKeyPair = sp_core::sr25519::Pair;
	type SignedTransaction = bp_template::UncheckedExtrinsic<template_runtime::Call>;

	fn sign_transaction(
		genesis_hash: <Self::Chain as ChainBase>::Hash,
		signer: &Self::AccountKeyPair,
		signer_nonce: <Self::Chain as Chain>::Index,
		call: <Self::Chain as Chain>::Call,
	) -> Self::SignedTransaction {
		let raw_payload = SignedPayload::new(
			call,
			bp_template::SignedExtensions::new(
				bp_template::VERSION,
				sp_runtime::generic::Era::Immortal,
				genesis_hash,
				signer_nonce,
				0,
			),
		)
		.expect("SignedExtension never fails.");

		let signature = raw_payload.using_encoded(|payload| signer.sign(payload));
		let signer: sp_runtime::MultiSigner = signer.public().into();
		let (call, extra, _) = raw_payload.deconstruct();

		bp_template::UncheckedExtrinsic::new_signed(
			call,
			sp_runtime::MultiAddress::Id(signer.into_account()),
			signature.into(),
			extra,
		)
	}
}

/// Template signing params.
pub type SigningParams = sp_core::sr25519::Pair;
