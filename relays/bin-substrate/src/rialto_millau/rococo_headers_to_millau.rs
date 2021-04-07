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

//! Rococo-to-Millau headers sync entrypoint.

use crate::finality_pipeline::{SubstrateFinalitySyncPipeline, SubstrateFinalityToSubstrate};

use bp_header_chain::justification::GrandpaJustification;
use codec::Encode;
use relay_millau_client::{Millau, SigningParams as MillauSigningParams};
use relay_rococo_client::{Rococo, SyncHeader as RococoSyncHeader};
use relay_substrate_client::{Chain, TransactionSignScheme};
use relay_utils::metrics::{FloatJsonValueMetric, MetricsParams};
use sp_core::{Bytes, Pair};

/// Rococo-to-Millau finality sync pipeline.
pub(crate) type RococoFinalityToMillau = SubstrateFinalityToSubstrate<Rococo, Millau, MillauSigningParams>;

impl SubstrateFinalitySyncPipeline for RococoFinalityToMillau {
	const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str = bp_rococo::BEST_FINALIZED_ROCOCO_HEADER_METHOD;

	type TargetChain = Millau;

	fn customize_metrics(params: MetricsParams) -> anyhow::Result<MetricsParams> {
		Ok(
			relay_utils::relay_metrics(finality_relay::metrics_prefix::<Self>(), params.address)
				// Polkadot/Kusama prices are added as metrics here, because atm we don't have Polkadot <-> Kusama
				// relays, but we want to test metrics/dashboards in advance
				.standalone_metric(FloatJsonValueMetric::new(
					"https://api.coingecko.com/api/v3/simple/price?ids=Polkadot&vs_currencies=usd".into(),
					"$.polkadot.usd".into(),
					"polkadot_price".into(),
					"Polkadot price in USD".into(),
				))
				.map_err(|e| anyhow::format_err!("{}", e))?
				.standalone_metric(FloatJsonValueMetric::new(
					"https://api.coingecko.com/api/v3/simple/price?ids=Kusama&vs_currencies=usd".into(),
					"$.kusama.usd".into(),
					"kusama_price".into(),
					"Kusama price in USD".into(),
				))
				.map_err(|e| anyhow::format_err!("{}", e))?
				.into_params(),
		)
	}

	fn transactions_author(&self) -> bp_millau::AccountId {
		self.target_sign.public().as_array_ref().clone().into()
	}

	fn make_submit_finality_proof_transaction(
		&self,
		transaction_nonce: <Millau as Chain>::Index,
		header: RococoSyncHeader,
		proof: GrandpaJustification<bp_rococo::Header>,
	) -> Bytes {
		let call = millau_runtime::BridgeGrandpaRococoCall::<
			millau_runtime::Runtime,
			millau_runtime::RococoGrandpaInstance,
		>::submit_finality_proof(header.into_inner(), proof)
		.into();

		let genesis_hash = *self.target_client.genesis_hash();
		let transaction = Millau::sign_transaction(genesis_hash, &self.target_sign, transaction_nonce, call);

		Bytes(transaction.encode())
	}
}