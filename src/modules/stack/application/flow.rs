/* src/modules/stack/application/flow.rs */

use anyhow::Result;

use crate::engine::contract::{ProcessingStep, TerminatorResult};
use crate::engine::executor;
use crate::modules::stack::application::container::Container;

pub async fn execute_l7(
	step: &ProcessingStep,
	container: &mut Container,
	parent_path: String,
) -> Result<TerminatorResult> {
	executor::execute_l7(step, container, parent_path).await
}
