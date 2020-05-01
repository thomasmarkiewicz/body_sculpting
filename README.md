[![](https://img.shields.io/github/v/release/thomasmarkiewicz/body_sculpting?include_prereleases&sort=semver)](https://github.com/thomasmarkiewicz/body_sculpting/releases)
[![](https://github.com/thomasmarkiewicz/bodysculpting/workflows/Build/badge.svg?branch=master)](https://github.com/thomasmarkiewicz/body_sculpting/actions)
[![](https://codecov.io/gh/thomasmarkiewicz/body_sculpting/branch/master/graph/badge.svg)](https://codecov.io/gh/thomasmarkiewicz/body_sculpting)

![Body Sculpting](https://raw.githubusercontent.com/thomasmarkiewicz/bodysculpting/master/assets/github_banner.png)

My hope is that one day this app can become a viable alternative for keeping track of all of your workouts. Right now it is in its early stages of development and will not preserve workout data between version upgrades - so use it for testing and evaluation purposes only.

### Overall Goals

- beautiful, simple, and easy to use
- multi-platform: Linux phones primarily, but also Linux, Mac, and Windows desktops
- offline first: store data locally on a device
- online account and logging in is not required for core functionality
- online account is available as an opt-in to backup, sync, and interact with fellow athletes

### Building and deploying

The primary target for this app are these Linux phones:

- [Librem5](https://puri.sm/products/librem-5/)
- [Pinephone](https://www.pine64.org/pinephone/)

(For Android/iOS see: https://github.com/thomasmarkiewicz/bodysculpting)

Both of these Linux phones are 64-bit ARM devices so the code must be compiled targeting `aarch64` architecture.  The app is then packaged and deployed to the device as a flatpak.

Read and follow instructions in the [HOWTO.md](/HOWTO.md) the first time you want to build and deploy to your phone. After that you can use the [build.sh](/build.sh) and [deploy.sh](/deploy.sh) scripts (modify deploy.sh for your device).

For development and when targeting your local host, you can use the regular cargo build commands, for example:
```
cargo run
```
