# Mief

[![Build Status on Travis](https://travis-ci.org/BMeu/Mief.svg?branch=master)](https://travis-ci.org/BMeu/Mief)
[![Build Status on AppVeyor](https://ci.appveyor.com/api/projects/status/xxukbycd0en6kvr8?svg=true)](https://ci.appveyor.com/project/BMeu/mief)
[![Codecov](https://codecov.io/gh/BMeu/Mief/branch/master/graph/badge.svg)](https://codecov.io/gh/BMeu/Mief)
[![License](https://img.shields.io/github/license/BMeu/Mief.svg)](README.md)

_Mief_ is a [_Pong_](https://en.wikipedia.org/wiki/Pong) clone written in [_Rust_](https://www.rust-lang.org/en-US/).

**WORK IN PROGRESS**

## Usage

1. Installation:
   1. [Install Rust](https://rustup.rs/)
   2. Download _Mief_: `git clone https://github.com/BMeu/Mief.git; cd Mief`
2. Run _Mief_: `cargo run --release`

## Instructions

The goal is pretty simple: prevent the ball from leaving the field on your side. Each player controls a handle
(player 1 the left one, player 2 the right one). Move the handle up and down to return the ball to the other player.

Currently, _Mief_ only supports an endless mode - just play as long as you want. The game starts immediately after
starting _Mief_, and if you miss a ball, the next one will start immediately in the center of the field. If you want to
start a completely new game, you will have to quit and restart _Mief_.

### Controls

* Player 1:
  * Up: `W`
  * Down: `S`
* Player 2:
  * Up: `Up`
  * Down: `Down`
* Quit: `Esc`

## Future

- [X] ~~Repeatedly increase ball and handle speeds~~
- [X] ~~Adjust field size when resizing the window~~
- [ ] Do not immediately start the game and new balls
- [ ] Customizable settings
- [ ] Add a main menu, in-game instructions, settings, ...
- [ ] Select game mode: endless, reach `x` points, difference of `x`, ...
- [ ] Add difficulties (e.g. speed of handles and ball, size of handles, multi-ball, freely move handles)
- [ ] Add a 4-player mode: square field, player on each side
- [ ] Add controller support
- [ ] Add computer players

## License

Unless stated otherwise, the following licenses apply:

### Source Code

`Mief` is licensed under either of

 * Apache License, Version 2.0, ([`LICENSE-APACHE`](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([`LICENSE-MIT`](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

#### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

### Assets

* Font [`Anonymous Pro`](https://fontlibrary.org/en/font/anonymous-pro) by Mark Simonson, 2009, is licensed under the 
  [SIL Open Font License](http://scripts.sil.org/OFL).
