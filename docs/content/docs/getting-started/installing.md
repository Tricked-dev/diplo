+++
title = "Installing diplo"
description = "Guide on how to install diplo."
date = 2021-05-01T08:20:00+00:00
updated = 2021-05-01T08:20:00+00:00
draft = false
weight = 20
sort_by = "weight"
template = "docs/page.html"

[extra]
lead = "Guide on how to install diplo."
toc = true
top = false
+++

- [All platforms](#all-platforms)
- [Windows](#windows)
  - [Installer](#installer)
  - [Exe](#exe)
- [Linux](#linux)
  - [Debian/Ubuntu based distro](#debianubuntu-based-distro)
  - [ArchLinux](#archlinux)
  - [Other Linux](#other-linux)
- [MacOs](#macos)

## All platforms

Install [rustup](https://rustup.rs/#), thats how we are going to be installing cargo and rust.

Then just run `cargo install diplo` and diplo should be installed on your system

## Windows

Download the windows.zip from the releases tab <https://github.com/Tricked-dev/diplo/releases/tag/v0.8.4>.

unzip this

#### Installer

Then just open the installer and diplo should be installed into your path.

#### Exe

If you don't want to use the installer you can still use the exe from the zip using the commandline.

## Linux

Download the linux.zip(the x86 is what most people are going to want to get) from the releases tab <https://github.com/Tricked-dev/diplo/releases/tag/v0.8.4>.

```
cd Downloads
tar -xvf diplo-*.tar.xz
```

#### Debian/Ubuntu based distro

```
cd diplo-<arch>-linux
sudo apt install -f diplo_<version>_amd64.deb
```

#### ArchLinux

```
cd diplo-<arch>-linux
makepkg -si
```

#### Other Linux

```
cd diplo-<arch>-linux
sudo mv diplo /bin/
```

## MacOs

Download the macos.zip from the releases tab <https://github.com/Tricked-dev/diplo/releases/tag/v0.8.4>.

unzip this

then just move the binary to your bin folder.
