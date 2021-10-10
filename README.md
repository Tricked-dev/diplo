# Diplo <!-- omit in toc -->

[![Rust](https://github.com/Tricked-dev/diplo/actions/workflows/binaries.yml/badge.svg)](https://github.com/Tricked-dev/diplo/actions/workflows/binaries.yml)

Diplo is a deno script runner made in rust.

- [Installing](#installing)
- [Features](#features)
	- [Script running](#script-running)
	- [Dependencies](#dependencies)
		- [Updating Dependencies](#updating-dependencies)
	- [Dotenv Support](#dotenv-support)
	- [Example Config](#example-config)

# Installing

You can download diplo from the [releases tab](https://github.com/Tricked-dev/diplo/releases) or install it using cargo install diplo

# Features

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

If you add dependencies object to the diplo.json file diplo will automatically create a file called deps.ts inside the .diplo folder.

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

> This only works on javascript and will cause type errors in typescript

```js
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
	}
}
```

## Donating <!-- omit in toc -->

You can support the project by donating to my xmr address `89prBkdG58KU15jv5LTbP3MgdJ2ikrcyu1vmdTKTGEVdhKRvbxgRN671jfFn3Uivk4Er1JXsc1xFZFbmFCGzVZNLPQeEwZc`

## License <!-- omit in toc -->

This project is licensed under the terms of the [Apache License 2.0](./LICENSE)
