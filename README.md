# Anathema

![Builds - Passing](https://img.shields.io/github/actions/workflow/status/togglebyte/anathema/rust.yml) ![Release](https://img.shields.io/github/v/tag/togglebyte/anathema)
![GitHub top language](https://img.shields.io/github/languages/top/togglebyte/anathema) ![GitHub commit activity](https://img.shields.io/github/commit-activity/w/togglebyte/anathema)
![GitHub License](https://img.shields.io/github/license/togglebyte/anathema) ![Crates.io Size](https://img.shields.io/crates/size/anathema)
![GitHub Repo stars](https://img.shields.io/github/stars/togglebyte/anathema) ![GitHub forks](https://img.shields.io/github/forks/togglebyte/anathema)
![Crates.io Total Downloads](https://img.shields.io/crates/d/anathema) ![GitHub Issues](https://img.shields.io/github/issues/togglebyte/anathema)
![GitHub Pull Requests](https://img.shields.io/github/issues-pr/togglebyte/anathema) ![Closed GitHub Pull Requests](https://img.shields.io/github/issues-pr-closed/togglebyte/anathema)
![Closed GitHub Issues](https://img.shields.io/github/issues-closed/togglebyte/anathema)

A TUI library with a custom template language and runtime.

**Note** Anathema should be considered alpha for now.

[Getting started](https://togglebyte.github.io/anathema-guide/)

```yml
hstack [width: 40, height: 10]
    // Left pane
    expand [factor: 1]
        border
            vstack
                for item in [1, 2, 3]
                    text "Item " item

    // Right pane
    expand [factor: 4]
        border
            expand
                text "This isn't where I parked my car!"
```
output
```
┌──────┐┌──────────────────────────────┐
│Item 1││This isn't where I parked my  │
│Item 2││car!                          │
│Item 3││                              │
└──────┘│                              │
        │                              │
        │                              │
        │                              │
        │                              │
        └──────────────────────────────┘
```
