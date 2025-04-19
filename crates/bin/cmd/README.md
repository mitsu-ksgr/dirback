Dirback - cmd
=============

The dirback command line tool.


## Backup path
if environment variable `DIRBACK_STORE_DIR` is set, use it's path.
if not set, use default directory.

- Linux: `~/.local/share/dirback`
- Windows: `%APPDATA%\dirback`
  - `C:\Users\USER_NAME\AppData\Roaming\dirback`


## Commands
- `help`, `-h`
  - Print help.
- `list <target-path>`
  - Register new target.
- `show <target-id>`
  - Show target information.
- `backup <target-id> [note]`
  - Take a backup of the target.
- `restore <target-id> <backup-id>`
  - Restore the target from the specified backup.
- `delete <target-id> <backup-id>`
  - Delete the backup.
  - This action cannot be undone.
- `delete-target <target-id>`
  - Delete the target.
  - The target's backups will also be deleted.
  - This action cannot be undone.

