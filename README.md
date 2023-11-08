# tauri-vite-app-demo

这是一个 [tauri2](https://next--tauri.netlify.app/) + [vite](https://vitejs.dev/) + [vue3](https://vuejs.org/) + [typescript](https://www.typescriptlang.org/)` 的模板项目。

集成了 [cz-git](https://cz-git.qbb.sh) 管理 git 提交、[release-it](https://github.com/release-it/release-it) 管理发布版本、[commitlint](https://github.com/conventional-changelog/commitlint) 验证提交信息合规性、[conventional-changelog](https://github.com/conventional-changelog/conventional-changelog) 生成变更日志、[husky](https://github.com/husky/husky) 管理 git hooks 等工具。

## Recommended IDE Setup

[VSCode](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur) + [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin).

## Type Support for `.vue` Imports in TS

TypeScript cannot handle type information for `.vue` imports by default, so we replace the `tsc` CLI with `vue-tsc` for type checking. In editors, we need [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin) to make the TypeScript language service aware of `.vue` types.

If the standalone TypeScript plugin doesn't feel fast enough to you, Volar has also implemented a [Take Over Mode](https://github.com/johnsoncodehk/volar/discussions/471#discussioncomment-1361669) that is more performant. You can enable it by the following steps:

1. Disable the built-in TypeScript Extension
   1. Run `Extensions: Show Built-in Extensions` from VSCode's command palette
   2. Find `TypeScript and JavaScript Language Features`, right click and select `Disable (Workspace)`
2. Reload the VSCode window by running `Developer: Reload Window` from the command palette.

## Customize configuration

See [Vite Configuration Reference](https://vitejs.dev/config/).

## Project Setup

```sh
pnpm install
```

### Compile and Hot-Reload for Development

```sh
pnpm dev
```

### Type-Check, Compile and Minify for Production

```sh
pnpm build
```

### Run Unit Tests with [Vitest](https://vitest.dev/)

```sh
pnpm test:unit
```

### Run End-to-End Tests with [Playwright](https://playwright.dev)

```sh
# Install browsers for the first run
npx playwright install

# When testing on CI, must build the project first
pnpm build

# Runs the end-to-end tests
pnpm test:e2e
# Runs the tests only on Chromium
pnpm test:e2e --project=chromium
# Runs the tests of a specific file
pnpm test:e2e tests/example.spec.ts
# Runs the tests in debug mode
pnpm test:e2e --debug
```

### Lint with [ESLint](https://eslint.org/)

```sh
pnpm lint
```

### Prettier with [Prettier](https://prettier.io/)

```sh
pnpm format
```

### Tauri Script

#### start dev

```sh
pnpm tauri:dev
```

#### build

```sh
pnpm tauri:build
```

### Generate changelog file

```sh
pnpm script:changelog
```

### Generate Release

```sh
pnpm release
```

## Tauri updater with [Tauri](https://next--tauri.netlify.app/next/guides/distribution/updater)

### :ferris_wheel: Setting for Github

1. Project -> Settings
2. Security -> Secrets and Variables -> Actions
3. Secrets -> new repository secret

```sh
TAURI_KEY_PASSWORD="your password"
TAURI_PRIVATE_KEY="your private key"
```

### 关于版本

2.0 预览版本随时都在改变，项目中的这个版本是测试通过的，不要轻易去改变 tauri 的版本，等待 2.0 正式版发布。

## rust

### upgrade project dependencies

[cargo-edit](https://github.com/killercup/cargo-edit) can upgrade project dependencies to lastest version.

install:

```bash
cargo install cargo-edit
```

upgrade:

```
cargo upgrade
```
