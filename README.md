
![License](https://img.shields.io/badge/License-MIT-blue.svg)
![Language](https://img.shields.io/badge/Language-Rust-orange.svg)
![Project](https://img.shields.io/badge/Project-Browser-green.svg)
![Status](https://img.shields.io/badge/Status-Active%20Development-informational)


## Rust Browser

a web browser built completely from scratch in rust

rusr browser is a learning-focused browser project that implements its own html parser, DOM tree, layout engine, renderer, downloads, tabs, bookmarks, history system, image loading, text selection, and more.

The goal of this project is to understand how real browsers work internally by building one from the ground up.

___


## features

This repository contains the source code for RustBrowser.

Current features include:

* html parsing
* dom tree
* layout tree
* custom renderer
* image loading
* hyperlink navigation
* downloads
* tabs
* bookmarks
* history navigation
* text selection
* copy to clipboard
* css text colors
* css font sizes
* tables
* lists
* headings
* paragraphs
* div elements
* span elements

___

## browser arch

rust browser follows a simplified browser arch:
```text
HTML
↓
DOM Parser
↓
DOM Tree
↓
Layout Tree
↓
Renderer
↓
Pixels
```

the browser separates:

* structure (dom)
* latout (layout tree)
* rendering (pixels)


which makes future development easier.

___


## tech stack

### language

* rust

### crates

* html5ever
* markup5ever_rcdom
* reqwest
* tokio
* pixels
* winit
* image

___

## running locally

clone the repository:

```bash
git clone https://github.com/cyberworrier8088/RustBrowser.git
```

enter the project:

```bash
cd RustBrowser
```

run:

```bash
cargo run
```

optimized build:

```bash
cargo run --release
```

## or windows only
you can download this broswer from here:

https://github.com/cyberworrier8088/RustBrowser/releases/tag/demo

___


## demo video (vedeio is broken becuse so sizey)

![video](https://cdn.hackclub.com/019eea60-c210-7e8a-a56d-761a05df3922/Recording%202026-06-21%20184009%20(1)%20(1).mp4)

___

## License

This project is licensed under the MIT License.
