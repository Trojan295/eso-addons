# ESO Addon Manager

[![Support Server](https://img.shields.io/discord/788487566310899722.svg?label=Discord&logo=Discord&colorB=7289da&style=for-the-badge)](https://discord.gg/B3ZGrcne)

This repository holds a simple command line ESO Addon Manager written in Rust. With it you can manage addons from [esoui.com](https://www.esoui.com/).

The list of addons you want to install is put in a single configuration file. This means you can save and share your addon configuration with a single file!

<!-- toc -->

- [Usage](#usage)
    * [Configuration](#configuration)
    * [Install new addon](#install-new-addon)
    * [Update installed addons](#update-installed-addons)
    * [List addons, show missing or unused addon dependencies](#list-addons-show-missing-or-unused-addon-dependencies)
    * [Remove addons](#remove-addons)
    * [Backup and share your addon configuration](#backup-and-share-your-addon-configuration)

<!-- tocstop -->

## Usage

[![asciicast](https://asciinema.org/a/9gq2aq7BabZYz1G9B95RebSvG.svg)](https://asciinema.org/a/9gq2aq7BabZYz1G9B95RebSvG?autoplay=1&speed=2)

### Configuration

Run:

```bash
eso-addons list
```

to generate the config file. The config file is in your user directory:
- Linux - `/home/<username>/.eso-addons.toml`
- Windows - `C:/Users/<username>/.eso-addons.toml`

If necessary, edit the `addonDir` parameter in the config file to the directory, where your ESO addons should be placed:
```toml
addonDir = "/home/damian/drive_c/users/user/My Documents/Elder Scrolls Online/live/AddOns" # edit this, if needed
```

### Install new addon

To install a new addon use the `eso-addons add` command:
```bash
❯ eso-addons add
✔ URL of the addon on esoui.com · https://www.esoui.com/downloads/info1536-ActionDurationReminder.html
✔ Is addon only a dependency? · No
🎊 Installed ActionDurationReminder!
```

### Update installed addons

In case you want to update the addons to the newest version execute `eso-addons update`:
```bash
❯ eso-addons update
✔ Updated ActionDurationReminder!
✔ Updated LibAddonMenu-2.0!
```

### List addons, show missing or unused addon dependencies

To list the status of all installed addons, show missing or unused dependencies use `eso-addons list`
```
❯ eso-addons list
+------------------------+-----------+
| Name                   | Status    |
+------------------------+-----------+
| ActionDurationReminder | INSTALLED |
| LibAddonMenu-2.0       | MISSING   |
+------------------------+-----------+
```

### Remove addons

To remove an addon use `eso-addons remove`:
```bash
❯ eso-addons remove
✔ Select addon to remove · ActionDurationReminder
✔ Uninstalled ActionDurationReminder!
```

There is also the `eso-addons clean` command, which can be used to remove addons, which are not managed by `eso-addons` (i.e. you installed them manually):
```bash
❯ eso-addons clean
🗑 Addons to remove:
- LibAddonMenu-2.0

✔ Do you want to remove these addons? · Yes

✓ LibAddonMenu-2.0 removed!
```

### Backup and share your addon configuration

Just backup the `eso-addons.toml` file and that's it! In case you have to restore the addons (e.g. after an OS reinstall), just put the backuped `eso-addons.toml` in [user directory](#configuration) and run `eso-addons update` to install all addons.

You can also share your addon configuration with other people by sending them your `eso-addons.toml` file.
