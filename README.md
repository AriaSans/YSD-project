# Tauri + SvelteKit + TypeScript

This template should help get you started developing with Tauri, SvelteKit and TypeScript in Vite.

## Recommended IDE Setup

[VS Code](https://code.visualstudio.com/) + [Svelte](https://marketplace.visualstudio.com/items?itemName=svelte.svelte-vscode) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer).

```
RAField
├─ assets
│  ├─ abc.txt
│  ├─ affix
│  │  └─ affix_common.json
│  ├─ data
│  │  ├─ affix
│  │  ├─ buffs
│  │  │  ├─ buff
│  │  │  └─ debuff
│  │  ├─ operators
│  │  └─ skills
│  └─ images
├─ README.md
├─ src
│  ├─ app.css
│  ├─ app.html
│  ├─ routes
│  │  ├─ +layout.ts
│  │  └─ +page.svelte
│  └─ static
│     └─ i18n
│        ├─ en-US.json
│        └─ zh-CN.json
├─ src-tauri
│  ├─ 2
│  ├─ capabilities
│  │  └─ default.json
│  ├─ Cargo.lock
│  ├─ Cargo.toml
│  ├─ gen
│  │  └─ schemas
│  │     ├─ acl-manifests.json
│  │     ├─ capabilities.json
│  │     ├─ desktop-schema.json
│  │     └─ windows-schema.json
│  ├─ icons
│  │  ├─ 128x128.png
│  │  ├─ 128x128@2x.png
│  │  ├─ 32x32.png
│  │  ├─ icon.icns
│  │  ├─ icon.ico
│  │  ├─ icon.png
│  │  ├─ Square107x107Logo.png
│  │  ├─ Square142x142Logo.png
│  │  ├─ Square150x150Logo.png
│  │  ├─ Square284x284Logo.png
│  │  ├─ Square30x30Logo.png
│  │  ├─ Square310x310Logo.png
│  │  ├─ Square44x44Logo.png
│  │  ├─ Square71x71Logo.png
│  │  ├─ Square89x89Logo.png
│  │  └─ StoreLogo.png
│  ├─ src
│  │  ├─ bin
│  │  │  └─ gen_affix_common.rs
│  │  ├─ command
│  │  ├─ command.rs
│  │  ├─ domain
│  │  │  ├─ combat_context
│  │  │  │  ├─ action.rs
│  │  │  │  ├─ environment.rs
│  │  │  │  ├─ runtime_operator.rs
│  │  │  │  └─ trigger_listener.rs
│  │  │  ├─ combat_context.rs
│  │  │  ├─ dependencybundle.rs
│  │  │  ├─ gamedb
│  │  │  │  ├─ buffs.rs
│  │  │  │  ├─ gears.rs
│  │  │  │  ├─ operators.rs
│  │  │  │  ├─ skills.rs
│  │  │  │  └─ weapons.rs
│  │  │  ├─ gamedb.rs
│  │  │  ├─ project_state
│  │  │  │  ├─ dependency.rs
│  │  │  │  ├─ environment_setting.rs
│  │  │  │  ├─ operator_instance.rs
│  │  │  │  └─ skill_instance.rs
│  │  │  ├─ project_state.rs
│  │  │  ├─ setting.rs
│  │  │  ├─ types
│  │  │  │  ├─ data.rs
│  │  │  │  ├─ fixed.rs
│  │  │  │  ├─ id.rs
│  │  │  │  ├─ statusbuff
│  │  │  │  │  ├─ enemy.rs
│  │  │  │  │  └─ operator.rs
│  │  │  │  ├─ statusbuff.rs
│  │  │  │  ├─ tick.rs
│  │  │  │  └─ trigger.rs
│  │  │  └─ types.rs
│  │  ├─ domain.rs
│  │  ├─ lib.rs
│  │  ├─ main.rs
│  │  ├─ services
│  │  ├─ services.rs
│  │  └─ state.rs
│  └─ tauri.conf.json
├─ static
│  ├─ favicon.png
│  ├─ svelte.svg
│  ├─ tauri.svg
│  └─ vite.svg
├─ svelte.config.js
├─ tsconfig.json
└─ vite.config.js

```
