//!
//! # ListTargets command
//!

use dirback::adapter::ListTargetsAdapter;
use dirback::infra::repository::in_memory::InMemoryTargetRepository;

pub struct ListTargets;

impl cmd::Command for ListTargets {
    fn execute(&self, _params: &cmd::CmdParams) -> anyhow::Result<()> {
        // TODO: Repository
        let repo = InMemoryTargetRepository::new();

        let list_targets = ListTargetsAdapter::new(&repo);
        let targets = list_targets.execute()?;

        println!("* Targets ({})", targets.len());
        for target in targets {
            println!("- {:?}", target);
        }

        Ok(())
    }
}
