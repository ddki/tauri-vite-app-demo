<div align="center" id="top"> 
  <!-- <img src="./.github/app.gif" alt="Tauri Vite App Template" /> -->

&#xa0;

  <!-- <a href="https://tauriviteappdemo.netlify.app">Demo</a> -->
</div>

<h1 align="center">Tauri Vite App Template</h1>

<p align="center">
  <img alt="Github release" src="https://img.shields.io/github/release/ddki/tauri-vite-app-template">

  <img alt="Github top language" src="https://img.shields.io/github/languages/top/ddki/tauri-vite-app-template">

  <!-- <img alt="Github language count" src="https://img.shields.io/github/languages/count/ddki/tauri-vite-app-template"> -->

  <img alt="Repository size" src="https://img.shields.io/github/repo-size/ddki/tauri-vite-app-template">

  <img alt="License" src="https://img.shields.io/github/license/ddki/tauri-vite-app-template">

  <!-- <img alt="Github issues" src="https://img.shields.io/github/issues/ddki/tauri-vite-app-template" /> -->

  <img alt="Github forks" src="https://img.shields.io/github/forks/ddki/tauri-vite-app-template" />

  <img alt="Github stars" src="https://img.shields.io/github/stars/ddki/tauri-vite-app-template" />
</p>

<!-- Status

<h4 align="center">
	🚧  Tauri Vite App Demo 🚀 Under construction...  🚧
</h4>

<hr> -->

<p align="center">
  <a href="#dart-about">About</a> &#xa0; | &#xa0; 
  <a href="#sparkles-features">Features</a> &#xa0; | &#xa0;
  <a href="#rocket-technologies">Technologies</a> &#xa0; | &#xa0;
  <a href="#white_check_mark-requirements">Requirements</a> &#xa0; | &#xa0;
  <a href="#checkered_flag-starting">Starting</a> &#xa0; | &#xa0;
  <a href="#memo-license">License</a> &#xa0; | &#xa0;
  <a href="https://github.com/ddki" target="_blank">Author</a>
</p>

<br>

## :dart: About

这是一个 [tauri2](https://next--tauri.netlify.app/) + [vite](https://vitejs.dev/) + [vue3](https://vuejs.org/) + [typescript](https://www.typescriptlang.org/)` 的模板项目。

集成了 [cz-git](https://cz-git.qbb.sh) 管理 git 提交、[release-it](https://github.com/release-it/release-it) 管理发布版本、[commitlint](https://github.com/conventional-changelog/commitlint) 验证提交信息合规性、[conventional-changelog](https://github.com/conventional-changelog/conventional-changelog) 生成变更日志、[husky](https://github.com/husky/husky) 管理 git hooks 等工具。

## :sparkles: Features

:heavy_check_mark: Vite + Vue3 + Tauri2;\
:heavy_check_mark: 支持 GitHub Action 自动发布;\
:heavy_check_mark: 集成 Release-it 发版工具;\
:heavy_check_mark: 内置菜单、独立窗口、任务栏图标、案例等;\
:black_square_button: 支持更新;

## :rocket: Technologies

The following tools were used in this project:

- [Tauri2](https://next--tauri.netlify.app/)
- [Vite](https://vitejs.dev/)
- [Vue3](https://vuejs.org/)
- [Node.js](https://nodejs.org/en/)
- [TypeScript](https://www.typescriptlang.org/)
- [cz-git](https://cz-git.qbb.sh)
- [release-it](https://github.com/release-it/release-it)
- [commitlint](https://github.com/conventional-changelog/commitlint)
- [conventional-changelog](https://github.com/conventional-changelog/conventional-changelog)
- [husky](https://github.com/husky/husky)

### Recommended IDE Setup

[VSCode](https://code.visualstudio.com/) + [Volar](https://marketplace.visualstudio.com/items?itemName=Vue.volar) (and disable Vetur) + [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin).

### Type Support for `.vue` Imports in TS

TypeScript cannot handle type information for `.vue` imports by default, so we replace the `tsc` CLI with `vue-tsc` for type checking. In editors, we need [TypeScript Vue Plugin (Volar)](https://marketplace.visualstudio.com/items?itemName=Vue.vscode-typescript-vue-plugin) to make the TypeScript language service aware of `.vue` types.

If the standalone TypeScript plugin doesn't feel fast enough to you, Volar has also implemented a [Take Over Mode](https://github.com/johnsoncodehk/volar/discussions/471#discussioncomment-1361669) that is more performant. You can enable it by the following steps:

1. Disable the built-in TypeScript Extension
   1. Run `Extensions: Show Built-in Extensions` from VSCode's command palette
   2. Find `TypeScript and JavaScript Language Features`, right click and select `Disable (Workspace)`
2. Reload the VSCode window by running `Developer: Reload Window` from the command palette.

## :white_check_mark: Requirements

Before starting :checkered_flag:, you need to have [Git](https://git-scm.com), [Node](https://nodejs.org/en/), [Rust](https://www.rust-lang.org/) installed.

### Tauri updater with [Tauri](https://next--tauri.netlify.app/next/guides/distribution/updater)

#### :ferris_wheel: Setting for Github

1. Project -> Settings
2. Security -> Secrets and Variables -> Actions
3. Secrets -> new repository secret

```sh
TAURI_KEY_PASSWORD="your password"
TAURI_PRIVATE_KEY="your private key"
```

### About Tauri Version

2.0 预览版本随时都在改变，项目中的这个版本是测试通过的，不要轻易去改变 tauri 的版本，等待 2.0 正式版发布。

### Upgrade project's rust dependencies

[cargo-edit](https://github.com/killercup/cargo-edit) can upgrade project dependencies to lastest version.

```bash
# install
cargo install cargo-edit

# upgrade
cargo upgrade
```

## :checkered_flag: Starting

```bash
# Clone this project
git clone https://github.com/ddki/tauri-vite-app-template

# Access
cd tauri-vite-app-template

# Install dependencies
pnpm install

# Run the project
pnpm run tauri:dev

# build the project
pnpm run tauri:build

# Generate changelog file
pnpm run changelog
# or
pnpm run script:changelog

# commit file to git
pnpm run commit

# release
pnpm run release

```

## Release Step

```bash
# step 1
git add -A
# step 2
pnpm commit
# step 3
pnpm release
```

## :memo: License

This project is under license from GPL-3.0. For more details, see the [LICENSE](LICENSE) file.

Made with :heart: by <a href="https://github.com/ddki" target="_blank">ddki</a>

&#xa0;

<a href="#top">Back to top</a>
