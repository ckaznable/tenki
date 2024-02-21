# Tenki

tty-clock with weather effect written by Rust and powerd by [ratatui](https://github.com/ratatui-org/ratatui)

![demo](./doc/demo.gif)

## Installation

### Install from Source Code

tenki is written in Rust, so you'll need to grab a [Rust installation](https://www.rust-lang.org/) in order to compile it.

```shell
git clone https://github.com/ckaznable/tenki
cd tenki
make build
make install
```

If you want to uninstall

```shell
make uninsall
```

### Install from the AUR

If you are using Arch Linux, you can install tenki using an [AUR helper](https://wiki.archlinux.org/title/AUR_helpers). For example:

```shell
paru -S tenki
```

## Usage

```
Usage: tenki [OPTIONS]

Options:
      --mode <MODE>    [default: rain] [possible values: rain, snow]
      --fps <FPS>      [default: 30] 1-60
      --color <COLOR>  [default: white] [red, green, blue, yellow, cyan, magenta, white, black]
  -l, --level <LEVEL>  effect level, The lower, the stronger [4-1000] [default: 50]
  -h, --help           Print help
  -V, --version        Print version
```

## Roadmap

- [x] CLI options
- [ ] customizable

## LICENSE

[MIT](./LICENSE)

