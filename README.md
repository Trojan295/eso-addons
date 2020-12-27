# ESO Addon Manager

This repository holds a simple command line ESO Addon Manager written in Rust. With it you can manage addons from [esoui.com](https://www.esoui.com/). It uses a single config file to configure the desired addons.

- [ESO Addon Manager](#eso-addon-manager)
  - [Usage](#usage)
    - [Configuration](#configuration)
    - [Install and update addon](#install-and-update-addon)
    - [List addons, show missing or unused dependencies](#list-addons-show-missing-or-unused-dependencies)
    - [Remove addons](#remove-addons)

## Usage

### Configuration

Create a config file in your user directory:
- Linux - `$HOME/.eso-addons.toml`

For a example config see [eso-addons.toml](./eso-addons.toml).

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
