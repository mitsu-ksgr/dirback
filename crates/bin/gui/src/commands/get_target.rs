//!
//! # GetTargets command
//!

use crate::commands::Command;

use dirback::adapter::GetTargetAdapter;
use dirback::infra::repository::file_storage::FileStorageTargetRepository;
use dirback::usecase::dto::Target;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GetTargetPayload {
    pub target_id: String,
}

pub struct GetTarget;

impl Command for GetTarget {
    type Payload = GetTargetPayload;
    type Output = Option<Target>;

    fn execute(
        &self,
        datadir: &std::path::Path,
        payload: Self::Payload,
    ) -> anyhow::Result<Self::Output> {
        if payload.target_id.is_empty() {
            return Ok(None);
        }

        let repo = FileStorageTargetRepository::new(datadir);
        let adapter = GetTargetAdapter::new(&repo);
        Ok(adapter.execute(&payload.target_id))
    }
}
