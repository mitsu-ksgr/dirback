Dirback - GUI
=============

GUI for the dirback.

This application is implemented mainly using the following frameworks.

- [Tauri 2.0](https://v2.tauri.app/)
- [Svelte](https://svelte.dev/)


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


