Dirback
=======

- A simple directory backup tool.


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
    - target dir path
- BackupFile path
  - `${APP_DATA_PATH}/targets/{TARGET_ID}/info.json`
  - `${APP_DATA_PATH}/targets/{TARGET_ID}/backups/{DATETIME}.tar.gz`

