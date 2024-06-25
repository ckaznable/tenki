# Tenki(天気)

tty-clock with weather effect written by Rust and powerd by [ratatui](https://github.com/ratatui-org/ratatui) and tenki means weather in japanese

![demo](./doc/demo.gif)

## Installation


[![Packaging status](https://repology.org/badge/vertical-allrepos/tenki.svg)](https://repology.org/project/tenki/versions)

### Install from Cargo

```
cargo install --git https://github.com/ckaznable/tenki.git
```

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
      --mode <MODE>                [default: rain] [possible values: rain, snow, meteor, disable]
      --timer-mode <TIMER_MODE>    [possible values: dvd]
      --timer-color <TIMER_COLOR>  color of the effect. [red, green, blue] [default: white]
  -f, --fps <FPS>                  frame per second [default: 60]
  -t, --tps <TPS>                  tick per second [default: 60]
  -l, --level <LEVEL>              effect level, The lower, the stronger [4-1000]
      --wind <WIND>                wind mode. [random, disable, only-right, only-left, right, left] [default: random]
      --show-fps                   show fps at right-top in screen
      --blink-colon                blinking colon of timer
  -h, --help                       Print help
  -V, --version                    Print version
```

## Roadmap

- [x] CLI options
- [ ] customizable

## LICENSE

[MIT](./LICENSE)

