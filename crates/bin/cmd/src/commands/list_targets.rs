//!
//! # ListTargets command
//!

use dirback::adapter::ListTargetsAdapter;
use dirback::infra::repository::file_storage::FileStorageTargetRepository;

pub struct ListTargets;

impl cmd::Command for ListTargets {
    fn execute(&self, params: &cmd::CmdParams) -> anyhow::Result<()> {
        let repo = FileStorageTargetRepository::new(&params.basedir);

        let list_targets = ListTargetsAdapter::new(&repo);
        let targets = list_targets.execute()?;

        println!("* Targets ({})", targets.len());
        for target in targets {
            println!(
                "{}, {}, {}, {}",
                target.id,
                target.name,
                target.path.to_string_lossy(),
                target.backups.len()
            );
        }

        Ok(())
    }
}
