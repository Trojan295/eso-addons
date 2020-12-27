# ESO Addon Manager

This repository holds a simple command line ESO Addon Manager written in Rust. With it you can manage addons from [esoui.com](https://www.esoui.com/). It uses a single config file to configure the desired addons.

- [ESO Addon Manager](#eso-addon-manager)
  - [Usage](#usage)
    - [Configuration](#configuration)
    - [Install and update addon](#install-and-update-addon)
    - [List addons and show missing dependencies](#list-addons-and-show-missing-dependencies)
    - [Remove addons](#remove-addons)

## Usage

### Configuration

Create a config file in your user directory:
- Linux - `$HOME/.eso-addons.toml`


```
# addonDir - path of the ESO addon directory
addonDir = "/home/damian/Games/the-elder-scrolls-online-tamriel-unlimited/drive_c/users/damian/My Documents/Elder Scrolls Online/live/AddOns"

# addons - list of addons to be installed
#   name - name of addon, must correspond to the addon directory name
#   url - download URL of the addon. It's the URL of the 'Download' button on ESOUI
#   TODO: dependency - is the addon only required as a dependency for another addon. Based on this Addon Manager will remove unused addons

[[addons]]
name = "SkyShards"
url = "https://www.esoui.com/downloads/download128-SkyShards.html"
dependency = false

[[addons]]
name = "HarvestMap"
url = "https://www.esoui.com/downloads/download57-HarvestMap.html"
dependency = false
```

### Install and update addon

Execute `eso-addons install`, to install and updated all the plugins.

### List addons and show missing dependencies

Execute `eso-addons list`. This will show the installation status of all addons and detect missing dependencies for the installed addons.

### Remove addons

The Addon Manager can remove addons, which are present in the ESO addon dir, but not listed in the configuration file.

To get the list of addons to be removed, execute:
```
eso-addons clean
```

To remove the addons, execute:
```
eso-addons clean --remove
```
