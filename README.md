# TF2 Classic Launcher

A tiny launcher for [Team Fortress 2 Classic](https://tf2classic.com/), a Source Engine mod seeking to reimagine the earlier days of Team Fortress 2.
Launches the game and passes through any arguments you send to it - that's it.

Currently only available for Windows. Not officially affiliated with TF2 Classic or Eminoma.

## Installation

To install the launcher, simply download the .exe file from the latest release of the launcher and run it anywhere.
It's recommended to keep the launcher in your TF2 Classic installation folder, but this is optional.

Requires TF2 Classic to be installed, which requires [Steam](https://store.steampowered.com/about/) and the Source 2013 Multiplayer SDK.
See [the TF2 Classic download page](https://tf2classic.com/download) for more details.

## Development

To build this launcher, use the `cargo build` command in the project's root directory.
The only supported target triples for this project are `i686-pc-windows-msvc` and `i686-pc-windows-gnu`; do not report issues if you are compiling for other targets.
