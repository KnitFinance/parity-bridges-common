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

//! Millau-to-Template headers sync entrypoint.

use crate::finality_pipeline::{SubstrateFinalitySyncPipeline, SubstrateFinalityToSubstrate};

use bp_header_chain::justification::GrandpaJustification;
use codec::Encode;
use relay_millau_client::{Millau, SyncHeader as MillauSyncHeader};
use relay_substrate_client::{Chain, TransactionSignScheme};
use relay_template_client::{SigningParams as TemplateSigningParams, Template};
use sp_core::{Bytes, Pair};

/// Millau-to-Template finality sync pipeline.
pub(crate) type MillauFinalityToTemplate = SubstrateFinalityToSubstrate<Millau, Template, TemplateSigningParams>;

impl SubstrateFinalitySyncPipeline for MillauFinalityToTemplate {
	const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str = bp_millau::BEST_FINALIZED_MILLAU_HEADER_METHOD;

	type TargetChain = Template;

	fn transactions_author(&self) -> bp_template::AccountId {
		(*self.target_sign.public().as_array_ref()).into()
	}

	fn make_submit_finality_proof_transaction(
		&self,
		transaction_nonce: <Template as Chain>::Index,
		header: MillauSyncHeader,
		proof: GrandpaJustification<bp_millau::Header>,
	) -> Bytes {
		let call = template_runtime::GrandpaCall::submit_finality_proof(header.into_inner(), proof).into();

		let genesis_hash = *self.target_client.genesis_hash();
		let transaction = Template::sign_transaction(genesis_hash, &self.target_sign, transaction_nonce, call);

		Bytes(transaction.encode())
	}
}
