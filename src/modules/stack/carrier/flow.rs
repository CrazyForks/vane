/* src/modules/stack/carrier/flow.rs */

use anyhow::Result;

use crate::modules::{
	flow::{context::TransportContext, engine},
	plugins::core::model::{ConnectionObject, ProcessingStep, TerminatorResult},
};
use crate::resources::kv::KvStore;

use bytes::Bytes;

pub async fn execute(
	step: &ProcessingStep,
	kv: &mut KvStore,
	conn: ConnectionObject,
	parent_path: String,
	initial_payloads: ahash::AHashMap<String, Bytes>,
) -> Result<TerminatorResult> {
	kv.insert("conn.layer".to_string(), "l4p".to_string());

	let mut context = TransportContext {
		kv,
		payloads: initial_payloads,
	};
	engine::execute(step, &mut context, conn, parent_path).await
}
