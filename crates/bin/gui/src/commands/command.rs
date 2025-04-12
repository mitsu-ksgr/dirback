//!
//! # Command
//!

pub trait Command {
    type Payload;
    type Output;

    fn execute(
        &self,
        datadir: &std::path::Path,
        payload: Self::Payload,
    ) -> anyhow::Result<Self::Output>;
}
