# ESO Addon Manager

[![Support Server](https://img.shields.io/discord/788487566310899722.svg?label=Discord&logo=Discord&colorB=7289da&style=for-the-badge)](https://discord.gg/B3ZGrcne)

This repository holds a simple command line ESO Addon Manager written in Rust. With it you can manage addons from [esoui.com](https://www.esoui.com/).

The list of addons you want to install, is put in a single configuration file. This means you can save and share your addon configuration with a single file!

- [ESO Addon Manager](#eso-addon-manager)
  - [Usage](#usage)
    - [Configuration](#configuration)
    - [Install and update addon](#install-and-update-addon)
    - [List addons, show missing or unused dependencies](#list-addons-show-missing-or-unused-dependencies)
    - [Remove addons](#remove-addons)
    - [Backup and share your addon configuration](#backup-and-share-your-addon-configuration)

## Usage

<a href="https://asciinema.org/a/381564" target="_blank"><img height="500px" src="https://asciinema.org/a/381564.svg" /></a>


### Configuration

Create a config file in your user directory:
- Linux - `$HOME/.eso-addons.toml`

For an example config see [eso-addons.toml](./eso-addons.toml).

### Install and update addon

To install or update addons, execute:
```
eso-addons update
```

### List addons, show missing or unused dependencies

To list the installation status of addons and show missing or unused dependencies, execute:
```
eso-addons list
```

### Remove addons

The Addon Manager can remove addons, which are present in the ESO addon dir, but not listed in the configuration file.

It can also detect addons, which are an unused dependency, i.e. you installed addon A, because it was a dependency of B. When you remove B, then `eso-addons` will detect that A is not required anymore and can be removed.

To get the list of addons to be removed, execute:
```
eso-addons clean
```

To remove the addons, execute:
```
eso-addons clean --remove
```

### Backup and share your addon configuration

Just backup the `eso-addons.toml` file and that's it! In case you have to restore the addons (e.g. after a OS reinstall), just use the backup `eso-addons.toml` and install the addons again.

You can also share your addon configuration with friends using the same way, by sending them your `eso-addons.toml` file.
