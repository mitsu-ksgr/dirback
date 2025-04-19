<div align="center">
  <img src="./crates/bin/gui/icons/128x128.png" alt="dirback logo" width="128" />
  <h1>Dirback</h1>
  <h3>Simple directory-based backup tool.</h3>

  ![Badge Workflow](https://github.com/mitsu-ksgr/dirback/actions/workflows/release.yml/badge.svg)
  ![Badge Workflow](https://github.com/mitsu-ksgr/dirback/actions/workflows/rust.yml/badge.svg)
  ![Badge Workflow](https://github.com/mitsu-ksgr/dirback/actions/workflows/gui-frontend.yml/badge.svg)
</div>


Dirback is a simple directory-based backup tool.

Back up the target directory as a `tar.gz` file.
it also allows for easy restoration from the backup.


## Apps
Dirback provides the following applications:

- [Command line tool](./crates/bin/cmd/README.md)
- [TUI](./crates/bin/tui/README.md)
- [GUI](./crates/bin/gui/README.md)

### Supported platforms
- [x] Linux
- [x] Windows
- [ ] macOS


## Application data path
Backup files will be stored in the following path:

- Linux: `~/.local/share/dirback`
- Windows: `%APPDATA%\dirback`
  - `C:\Users\USER_NAME\AppData\Roaming\dirback`


## License
[MIT](./LICENSE)

