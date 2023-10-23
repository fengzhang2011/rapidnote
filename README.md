### RapidNote - A rapid way to take notes.

#### Steps

```bash
sudo apt install curl
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh <(curl https://create.tauri.app/sh)
```

Output:
```bash
Your system is missing dependencies (or they do not exist in $PATH):
╭────────────────────┬──────────────────────────────────────────────────────────────────────────────────╮
│ Tauri CLI          │ Run `cargo install tauri-cli`                                                    │
├────────────────────┼──────────────────────────────────────────────────────────────────────────────────┤
│ Trunk              │ Run `cargo install trunk`                                                        │
├────────────────────┼──────────────────────────────────────────────────────────────────────────────────┤
│ wasm32 target      │ Run `rustup target add wasm32-unknown-unknown`                                   │
├────────────────────┼──────────────────────────────────────────────────────────────────────────────────┤
│ webkit2gtk & rsvg2 │ Visit https://tauri.app/v1/guides/getting-started/prerequisites#setting-up-linux │
╰────────────────────┴──────────────────────────────────────────────────────────────────────────────────╯

Make sure you have installed the prerequisites for your OS: https://tauri.app/v1/guides/getting-started/prerequisites, then run:
  cd rapidnote
  cargo tauri dev
```

```bash
cargo install tauri-cli
rustup target add wasm32-unknown-unknown
cargo install --locked trunk
sudo apt install libjavascriptcoregtk-4.0-dev
sudo apt install libsoup2.4-dev
sudo apt install libpango1.0-dev
sudo apt install libatk1.0-dev
sudo apt install libgdk-pixbuf2.0-dev
sudo apt install librust-gdk-dev
sudo apt install libwebkit2gtk-4.0-dev
sudo apt install librsvg2-dev
sudo apt install libfuse-dev
```


```bash
sh <(curl https://create.tauri.app/sh)
yarn
Vue
yarn add element-plus
yarn add @element-plus/icons-vue
yarn add -D unplugin-vue-components unplugin-auto-import
yarn
yarn tauri dev
```

vite.config.js:
```javascript
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import AutoImport from 'unplugin-auto-import/vite'
import Components from 'unplugin-vue-components/vite'
import { ElementPlusResolver } from 'unplugin-vue-components/resolvers'

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    vue(),
    AutoImport({ resolvers: [ElementPlusResolver()] }),
    Components({ resolvers: [ElementPlusResolver()] })
  ],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
  },
  // 3. to make use of `TAURI_DEBUG` and other env variables
  // https://tauri.app/v1/api/config#buildconfig.beforedevcommand
  envPrefix: ["VITE_", "TAURI_"],
}));
```


##### Tauri + Vue 3

This template should help get you started developing with Tauri + Vue 3 in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

###### Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
