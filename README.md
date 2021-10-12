<p align="center" width="200" height="400">
	<br>
    <img src="assets/diplo_small.svg" width="auto" height="100">
	<br>
 <p align="center">
 
<!-- # Diplo omit in toc -->

[![GitHub issues](https://img.shields.io/github/issues/tricked-dev/diplo?style=for-the-badge)](https://github.com/tricked-dev/diplo/issues)[![GitHub license](https://img.shields.io/github/license/Tricked-dev/diplo?style=for-the-badge)](https://github.com/Tricked-dev/diplo/blob/main/LICENSE)[![Crates.io](https://img.shields.io/crates/d/diplo?label=crate%20downloads&style=for-the-badge)](https://crates.io/crates/diplo/)[![GitHub all releases](https://img.shields.io/github/downloads/tricked-dev/diplo/total?label=github%20downloads&style=for-the-badge)](https://github.com/Tricked-dev/diplo/releases/tag/v0.3.1)[![Discord](https://img.shields.io/discord/748956745409232945?logo=discord&style=for-the-badge)](https://discord.gg/mY8zTARu4g)

### Diplo is a script runner and dependency manager made in rust mainly for [Deno](https://deno.land/).

- [Installing](#installing)
- [Features](#features)
  - [File watching](#file-watching)
  - [Easy dependencies](#easy-dependencies)
  - [Script running](#script-running)
  - [Dependencies](#dependencies)
    - [Updating Dependencies](#updating-dependencies)
  - [Dotenv Support](#dotenv-support)
  - [Example Config](#example-config)
  - [Help menu](#help-menu)

# Installing

You can download diplo from the [releases tab](https://github.com/Tricked-dev/diplo/releases) or install it using `cargo install diplo`

After that you can run `diplo init` for a interactive setup.  
or use `diplo init -y` for a one command setup

# Features

## File watching

Diplo can replace [Denon](https://github.com/denosaurs/denon) in terms of restarting on file change.  
To restart a script on save you just have to append `--watch` to `diplo run <script>`

```sh
$ diplo run <script> --watch
```

## Easy dependencies

Adding a new dependency is as simple as running `diplo add natico` or whatever else you need

![Add](assets/add.png)

## Script running

You can easily create scripts like you do with npm and yarn

if you want to run the script just do `diplo run start` to run the start script.

```json
{
	"scripts": {
		"start": "deno run -A mod.ts",
		"node": "node index.js"
	}
}
```

## Dependencies

Diplo will automatically create a deps.ts file in the .diplo folder if you have dependencies specified in the diplo.json file

```json
{
	"dependencies": {
		"natico": "https://deno.land/x/natico/mod.ts",
		"server": "https://deno.land/std@0.110.0/http/server.ts"
	}
}
```

### Updating Dependencies

updating dependencies is a as simple as running `diplo update` note this will only update `deno.land/x/` packages

### Import Map Support <!-- omit in toc -->

> This will cause type errors in typescript but works perfectly fine with javascript.

```ts
import * as server from 'server';
```

- note Diplo will automatically append `--import-map="./.diplo/import_map.json` after `deno run`.

```json
{
	"import_map": true
}
```

## Dotenv Support

Diplo can automatically add environment variables using the rust dotenv module instead of the deno based one

```json
{
	"load_env": true
}
```

## Example Config

```json
{
	"scripts": {
		"test": "deno run -A mod.ts",
		"build": "deno bundle a.ts"
	},
	"import_map": false,
	"load_env": true,
	"dependencies": {
		"natico": "https://deno.land/x/natico/mod.ts"
	},
	"watcher": {
		"directory": ".",
		"clear": true
	}
}
```

## Help menu

```
$ diplo --help
diplo 0.3.0

Tricked-dev

Diplo is a script runner and dependency manager made in rust

USAGE:
    diplo [SUBCOMMAND]

FLAGS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add        Add a deno.land/x/ module
    help       Print this message or the help of the given subcommand(s)
    init       Initialize diplo
    install    This creates the .diplo directory with all required files
    run        Run a diplo script
    update     This updates all deno.land/x/ modules to their latest version
```

## Donating <!-- omit in toc -->

You can support the project by donating to my xmr address `89prBkdG58KU15jv5LTbP3MgdJ2ikrcyu1vmdTKTGEVdhKRvbxgRN671jfFn3Uivk4Er1JXsc1xFZFbmFCGzVZNLPQeEwZc`

## License <!-- omit in toc -->

This project is licensed under the terms of the [Apache License 2.0](./LICENSE)
