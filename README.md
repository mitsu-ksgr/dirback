Dirback
=======

- A simple directory backup tool.


## Todo
- Fix the paths in tests to be runnable cross platform.


## Notes
### Base path
- `APP_DATA_PATH`
  - Linux: "~/.local/share/dirback"
  - Windows: "TODO"

### Data path
- Target path
  - `/path/to/target/dir`
- Application data path
  - `${APP_DATA_PATH}/targets.json`
    - target id (uuid?)
- BackupFile path
  - `${APP_DATA_PATH}/targets/{TARGET_ID}/info.json`
    - target_id ... (uuid)
    - target_dir_path
    - backups
      - id ... backup id
      - filename ... (`{BACKUP_ID}_{TIMESTAMP}.tar.gz`)
      - note
  - `${APP_DATA_PATH}/targets/{TARGET_ID}/backups/{BACKUP_ID}_{TIMESTAMP}.tar.gz`


