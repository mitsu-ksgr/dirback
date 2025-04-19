Dirback - GUI
=============

GUI for the dirback.

This application is implemented mainly using the following frameworks.

- [Tauri 2.0](https://v2.tauri.app/)
- [Svelte](https://svelte.dev/)


## Support platforms
- [x] Linux
- [x] Windows 10~
- [ ] macOS



## Development
### Dependencies
- Docker
- [Dependencies required by Tauri](https://v2.tauri.app/start/prerequisites/#system-dependencies)
- [tauri-cli](https://v2.tauri.app/ja/reference/cli/)
  - `cargo install tauri-cli`


### Quick start
```sh
% cd ./crates/bin/gui

# Run front-end server
% docker-compose up -d

# Run the tauri app
% cargo tauri dev
```


### Front-end development
In the development, Run vite in a docker container.

The tauri application accesses Vite on the container
to fetch the front-end part (HTML/CSS/JS).

```sh
# Run front-end server
% docker-compose build
% docker-compose up -d
```


### Run the tauri application
Start the tauri application with `tauri-cli`.

```sh
% cargo tauri dev
```


### Without Docker
If you want to run vite on host machine,
change `"beforeDevCommand"` in `tauri.conf.json` as following:

```
   "version": "0.1.0",
   "identifier": "com.gui.app",
   "build": {
-    "beforeDevCommand": "",
+    "beforeDevCommand": "cd frontend && yarn dev",
     "devUrl": "http://localhost:1420",
     "beforeBuildCommand": "cd frontend && yarn build",
     "frontendDist": "./frontend/build"
```

and run `yarn tauri dev` instead `cargo tauri dev`.


## Front-end Development
### Tech stack
- Package manager: [yarn](https://yarnpkg.com/)
- Build tool: [Vite](https://vite.dev/)
- Language: TypeScript
- Framework: [Svelte5](https://svelte.dev/)
- CSS Framework: [Pico CSS](https://picocss.com/)
- Tools
  - Linter: [Oxc: oxlint](https://oxc.rs/docs/guide/usage/linter)
  - Formatter: [Prettier](https://prettier.io/)

```
% docker-compose run --rm app /bin/bash

# svelte-check
/app$ yarn run check

# linter: oxlint
/app$ yarn oxlint

# formatter: prettier
/app$ yarn prettier --write .
```

### Without tauri app
Develop the front-end without connecting the Tauri app (Rust side).

Create `.env` file and set `VITE_USE_MOCK_DISPATCHER=true`.

```
% cd crates/bin/gui/frontend
% cp .env.example .env
```

If you want to run release build, create `.env.production`.


## Icon
- Icon source ... `./icon-src`

Each icons can be generated using `cargo tauri icon` command.

```
% cd ./crates/bin/gui
% cargo tauri icon ./icon-src/icon.png

#> ./icons
```

Remove Android/iOS icons.

```
% rm -rf icons/android
% rm -rf icons/ios
```



