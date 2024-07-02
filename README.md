| Modern style | Classic style |
| :-: | :-: |
| <picture><source media="(prefers-color-scheme: dark)" srcset="repository/main-modern-dark.png"><img src="repository/main-modern.png"></picture> | <picture><source media="(prefers-color-scheme: dark)" srcset="repository/main-classic-dark.png"><img src="repository/main-classic.png"></picture> |
| <picture><source media="(prefers-color-scheme: dark)" srcset="repository/settings-modern-dark.png"><img src="repository/settings-modern.png"></picture> | <picture><source media="(prefers-color-scheme: dark)" srcset="repository/settings-classic-dark.png"><img src="repository/settings-classic.png"></picture> |

<p align="center">
    <a href="https://discord.gg/ck37X6UWBp">Discord</a> ·
    <a href="https://github.com/an-anime-team/wavey-launcher/wiki">Wiki</a>
</p>

<br>

<h2 align="center">⚠️ Game is not playable at the current state of the launcher ⚠️</h2>

Due to expected changes from the game's side, current launcher version doesn't support its anti-cheat. I'm not going to update it to support temporary workarounds.
Manualy launching methods could be found in our discord server. Please don't use this app until it's updated.

Repository will be archived until proper solution is found.

<br>

# ♥️ Useful links and thanks

* Original patch project without which this project wouldn't be possible. Link is omitted for "privacy" purposes
* [macOS launcher](https://github.com/3Shain/yet-another-anime-game-launcher) which contains some additional compatibility components
* [Wiki](https://github.com/an-anime-team/wavey-launcher/wiki) contains some basic FAQ, installation instructions and more
* [Releases page](https://github.com/an-anime-team/wavey-launcher/releases) where you can find latest available version
* [Changelog](CHANGELOG.md) with chronology of the project

<br>

# ⬇️ Download

| Distribution | Format | Wiki | Source |
| - | - | - | - |
| Any | Flatpak | [link](https://github.com/an-anime-team/wavey-launcher/wiki/Installation#-any-distribution-flatpak) | - |
| Arch Linux, Manjaro | AUR | [link](https://github.com/an-anime-team/wavey-launcher/wiki/Installation#-arch-linux-aur) | [wavey-launcher-bin](https://aur.archlinux.org/packages/wavey-launcher-bin) |
| Fedora, OpenSUSE | RPM | [link](https://github.com/an-anime-team/wavey-launcher/wiki/Installation#-fedora-rpm) | [HRL](https://build.opensuse.org/repositories/home:Maroxy:AAT-Apps/HRL) |
| Gentoo | ebuild | [link](https://github.com/an-anime-team/wavey-launcher/wiki/Installation#-gentoo-linux-ebuild) | [gentoo-ebuilds](https://github.com/an-anime-team/gentoo-ebuilds) |
| NixOS | nixpkg | [link](https://github.com/an-anime-team/wavey-launcher/wiki/Installation#-nixos-nixpkg) | [aagl-gtk-on-nix](https://github.com/ezKEa/aagl-gtk-on-nix) |

To see the installation guides, please visit the wiki page [here](https://github.com/an-anime-team/wavey-launcher/wiki/Installation)

Lutris integration described [here](https://github.com/an-anime-team/wavey-launcher/wiki/Installation#-lutris)

## Chinese version support

This should be automatically enabled if you're using zh_cn (Chinese) as your system language. If you're not using it - you can change the game edition in the launcher settings

<br>

# 💻 Development

| Folder | Description |
| - | - |
| src | Rust source code |
| assets | App assets folder |
| assets/locales | App localizations |
| target/release | Release build of the app |

## Clone repo

```sh
git clone --recursive https://github.com/an-anime-team/wavey-launcher
```

## Run app

```sh
cargo run
```

## Build app

```sh
cargo build --release
```

## Updates strategy

Starting from 3.2.1 ([fcab428](https://github.com/an-anime-team/wavey-launcher/commit/fcab428cb40b1457f41e0856f9d1e1473acbe653)) we have 2 branches: stable ([main](https://github.com/an-anime-team/wavey-launcher/tree/main)) and dev ([next](https://github.com/an-anime-team/wavey-launcher/tree/next)). Code changes will be pushed into dev branch and merged into stable once they're ready for new version release

<img src="repository/branches.png" />
