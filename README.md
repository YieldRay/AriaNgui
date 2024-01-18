# AriaNgui

Package `aria2c.exe` and `AriaNg` using `tauri` (currently only for Windows)

Note that aria2c will read the default [configuration file](https://aria2.github.io/manual/en/html/aria2c.html#aria2-conf) `%USERPROFILE%\.config\aria2`,  
where you can put extra config rule there.

## build

Make sure rust/cargo and node.js preinstalled.

```sh
# run these command in git bash
mkdir dist src-tauri/binaries
npm install
npm run update
npm run tauri build
```
