# the-life-engine-rust

A port of emergent garden's [The Life Engine](https://thelifeengine.net/)

This port is not feature complete, but the proof of concept here exists!

Currently missing:

- [ ] The ability to save and load simulations
- [ ] The ability to change the rules of the simulation
- [ ] The ability to change the size of the simulation
- [ ] Walls, food placement, custom organism placement
- [ ] simulation speed
- [ ] correct AI behavior
- [ ] organism ability to rotate

## Instructions to build this simulation

### Install Rust

Install rust using [this link](https://www.rust-lang.org/tools/install).

### If on Linux

install the [dependencies for bevy](https://github.com/bevyengine/bevy/blob/latest/docs/linux_dependencies.md):

The commands for ubuntu are

```shell
sudo apt-get install g++ pkg-config libx11-dev libasound2-dev libudev-dev libxkbcommon-x11-0
```

After your dependencies are installed, restart your shell.

- if you get the error `error: failed to run custom build command for libudev-sys v0.1.4`, it means that `libudev-dev` cannot be found. this can be resolved by restarting your computer if restarting your shell does not address this issue.

### If on Windows

install the [dependencies for bevy as described in the bevy quickstart]:

### After installing OS dependencies

```shell
rustup default nightly
cargo run --release
```

### Troubleshooting

Message @adamime on discord for help!
