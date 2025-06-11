# TwoMover3Pos

A utility written in [Rust](https://www.rust-lang.org/) (yes, it's 🚀🚀🚀BLAZING FAST🚀🚀🚀) using [egui](https://www.egui.rs/) to position momoving head lights in 3D space on EOS v2 lighting desks.


## Why that name?
The name came about form merging "EOS v**2** **mover** **3**D **pos**itioning"

## Roadmap
- [ ] V1
  - Take in coordinates of mover & line
  - Export the numbers to manually type in to desk
- [ ] V2
  - Automatically export cues to the desk using [OSC](https://en.wikipedia.org/wiki/Open_Sound_Control)
- [ ] V3
  - Import 3D splines to move lights along
  - Use hotkey to trace lights along path (to be bound to a rotary encoder)
- [ ] V4
  - 3D preview of lighting paths
## Dev envirionment setup

Run this project just like any other Rust project, with
```
cargo run
```

### Dependancies
[egui has some depndancies](https://github.com/emilk/egui?tab=readme-ov-file#demo), here's how to install them:

Debian:
```
sudo apt-get install libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev libxkbcommon-dev libssl-dev
```

Fedora Rawhide:

```
dnf install clang clang-devel clang-tools-extra libxkbcommon-devel pkg-config openssl-devel libxcb-devel gtk3-devel atk fontconfig-devel
```

For NixOS, there is a `flake.nix` file included, so you just need to run
```
use flake
```
