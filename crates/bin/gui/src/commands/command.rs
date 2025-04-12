//!
//! # Command
//!

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NoPayload;

pub trait Command {
    type Payload;
    type Output;

    fn execute(
        &self,
        datadir: &std::path::Path,
        payload: Self::Payload,
    ) -> anyhow::Result<Self::Output>;
}
